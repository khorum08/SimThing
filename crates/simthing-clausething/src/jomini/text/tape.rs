// Vendored from github.com/rakaly/jomini @ v0.34.1 (commit fff00d8c7f8f06c084d776d1a2c98b34324e64ed)
// License: MIT - see crates/simthing-clausething/src/jomini/LICENSE
use crate::jomini::{Error, ErrorKind, Scalar};
use crate::jomini::{
    Utf8Encoding, Windows1252Encoding,
    data::is_boundary,
    text::{ObjectReader, Operator},
};

/// Represents a valid text value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextToken<'a> {
    /// Start of an array
    Array {
        /// The index of the `TextToken::End` for this array
        end: usize,

        /// If this array contains a `MixedContainer` token
        mixed: bool,
    },

    /// Start of an object
    ///
    /// The value of a property typically immediately follows a key token. However,
    /// this is not guaranteed so always check if the end has been reached before
    /// trying to decode a value. There are two main situations where this is not
    /// guaranteed:
    ///
    /// - A non-equal operator (eg: `a > b` will be parsed to 3 instead of 2 tokens)
    /// - If an object switches to a mixed container that is both an array and object
    Object {
        /// Index of the `TextToken::End` that signifies this objects's termination
        end: usize,

        /// If this object contains a `MixedContainer` token
        mixed: bool,
    },

    /// Denotes the start of where a homogenous object or array becomes
    /// heterogenous.
    MixedContainer,

    /// Extracted unquoted scalar value
    Unquoted(Scalar<'a>),

    /// Extracted quoted scalar value
    Quoted(Scalar<'a>),

    /// A parameter scalar
    ///
    /// Only seen so far in EU4. From the patch notes:
    ///
    /// > Scripted triggers or effects now support conditional compilation on arguments provided to them.
    /// > You can now check for if an argument is defined or not and make the script look entirely different based on that.
    /// > Syntax is `[[var_name] code here ]` for if variable is defined
    ///
    /// ```ignore
    /// generate_advisor = { [[scaled_skill] if = { } ] }
    /// ```
    Parameter(Scalar<'a>),

    /// An undefined parameter, see Parameter variant for more info.
    ///
    /// Syntax for undefined variable:
    ///
    /// ```ignore
    /// [[!var_name] code here ]
    /// ```
    UndefinedParameter(Scalar<'a>),

    /// A present, but non-equal operator token
    Operator(Operator),

    /// Index of the start of this object
    End(usize),

    /// The header token of the subsequent scalar. For instance, given
    ///
    /// ```ignore
    /// color = rgb { 100 200 50 }
    /// ```
    ///
    /// `rgb` would be a the header followed by a 3 element array
    Header(Scalar<'a>),
}

impl<'a> TextToken<'a> {
    /// Returns the scalar if the token contains a scalar
    ///
    /// ```
    /// use jomini::{Scalar, TextToken};
    /// assert_eq!(TextToken::Unquoted(Scalar::new(b"abc")).as_scalar(), Some(Scalar::new(b"abc")));
    /// assert_eq!(TextToken::Quoted(Scalar::new(b"abc")).as_scalar(), Some(Scalar::new(b"abc")));
    /// assert_eq!(TextToken::Header(Scalar::new(b"rgb")).as_scalar(), Some(Scalar::new(b"rgb")));
    /// assert_eq!((TextToken::Object { end: 2, mixed: false }).as_scalar(), None);
    /// ```
    pub fn as_scalar(&self) -> Option<Scalar<'a>> {
        match self {
            TextToken::Header(s)
            | TextToken::Unquoted(s)
            | TextToken::Quoted(s)
            | TextToken::Parameter(s)
            | TextToken::UndefinedParameter(s) => Some(*s),
            _ => None,
        }
    }
}

/// Creates a parser that a writes to a text tape
#[derive(Debug, Default)]
pub struct TextTapeParser;

impl TextTapeParser {
    /// Create a text parser
    pub fn new() -> Self {
        TextTapeParser
    }

    /// Parse the text format and return the data tape
    pub fn parse_slice(self, data: &[u8]) -> Result<TextTape<'_>, Error> {
        let mut res = TextTape::default();
        self.parse_slice_into_tape(data, &mut res)?;
        Ok(res)
    }

    /// Parse the text format into the given tape.
    pub fn parse_slice_into_tape<'a>(
        self,
        data: &'a [u8],
        tape: &mut TextTape<'a>,
    ) -> Result<(), Error> {
        let token_tape = &mut tape.token_tape;
        token_tape.clear();
        token_tape.reserve(data.len() / 5);
        let mut state = ParserState {
            data,
            original_length: data.len(),
            token_tape,
            utf8_bom: false,
        };

        state.parse()?;
        tape.utf8_bom = state.utf8_bom;

        Ok(())
    }
}

struct ParserState<'a, 'b> {
    data: &'a [u8],
    original_length: usize,
    token_tape: &'b mut Vec<TextToken<'a>>,
    utf8_bom: bool,
}

/// Houses the tape of tokens that is extracted from plaintext data
#[derive(Debug, Default)]
pub struct TextTape<'a> {
    token_tape: Vec<TextToken<'a>>,
    utf8_bom: bool,
}

impl<'a> TextTape<'a> {
    /// Creates a windows 1252 object reader from the parsed tape
    pub fn windows1252_reader(&self) -> ObjectReader<'a, '_, Windows1252Encoding> {
        ObjectReader::new(self, Windows1252Encoding::new())
    }

    /// Creates a utf-8 object reader from the parsed tape
    pub fn utf8_reader(&self) -> ObjectReader<'a, '_, Utf8Encoding> {
        ObjectReader::new(self, Utf8Encoding::new())
    }
}

#[derive(Debug, PartialEq)]
enum ParseState {
    Key,
    KeyValueSeparator,
    ObjectValue,
    ArrayValue,
    ParseOpen,
}

/// I'm not smart enough to figure out the behavior of handling escape sequences when
/// when scanning multi-bytes, so this fallback is for when I was to reset and
/// process bytewise. It is much slower, but escaped strings should be rare enough
/// that this shouldn't be an issue
fn parse_quote_scalar_fallback(d: &[u8]) -> Result<(Scalar<'_>, &[u8]), Error> {
    let mut pos = 1;
    while pos < d.len() {
        if d[pos] == b'\\' {
            pos += 2;
        } else if d[pos] == b'"' {
            let scalar = Scalar::new(&d[1..pos]);
            return Ok((scalar, &d[pos + 1..]));
        } else {
            pos += 1;
        }
    }

    Err(Error::eof())
}

#[cfg(not(target_arch = "x86_64"))]
fn parse_quote_scalar(d: &[u8]) -> Result<(Scalar<'_>, &[u8]), Error> {
    use crate::jomini::util::{contains_zero_byte, repeat_byte};
    let sd = &d[1..];
    unsafe {
        let start_ptr = sd.as_ptr();
        let end_ptr = start_ptr.add(sd.len() / 8 * 8);

        let mut ptr = start_ptr;
        while ptr < end_ptr {
            let acc = (ptr as *const u64).read_unaligned();
            if contains_zero_byte(acc ^ repeat_byte(b'\\')) {
                break;
            } else if contains_zero_byte(acc ^ repeat_byte(b'"')) {
                while *ptr != b'"' {
                    ptr = ptr.offset(1);
                }

                let offset = sub(ptr, start_ptr);
                let (scalar, rest) = sd.split_at(offset);
                let s = Scalar::new(scalar);
                return Ok((s, &rest[1..]));
            }
            ptr = ptr.offset(8);
        }
    }

    parse_quote_scalar_fallback(d)
}

#[cfg(target_arch = "x86_64")]
fn parse_quote_scalar(d: &[u8]) -> Result<(Scalar<'_>, &[u8]), Error> {
    #[target_feature(enable = "sse2")]
    unsafe fn inner(d: &[u8]) -> Result<(Scalar<'_>, &[u8]), Error> {
        unsafe {
            // This is a re-implementation of memchr for a few reasons:
            //   - We maintain zero dependencies
            //   - memchr is optimized for finding a needle in a large haystack so we don't need the
            //   following performance improvements from memchr:
            //     - avx2 (there's no perf difference between sse2 and avx2 for our input)
            //     - aligned loads (we use unaligned)
            //     - loop unrolling
            use core::arch::x86_64::*;
            let haystack = &d[1..];
            let start_ptr = haystack.as_ptr();
            let mut ptr = start_ptr;
            let loop_size = std::mem::size_of::<__m128i>();
            let end_ptr = start_ptr.add(haystack.len() / loop_size * loop_size);
            let quote = _mm_set1_epi8(b'"' as i8);
            let slash = _mm_set1_epi8(b'\\' as i8);

            while ptr < end_ptr {
                let reg = _mm_loadu_si128(ptr as *const __m128i);
                let slash_found = _mm_cmpeq_epi8(slash, reg);
                if _mm_movemask_epi8(slash_found) != 0 {
                    break;
                }

                let quote_found = _mm_cmpeq_epi8(quote, reg);
                let mask = _mm_movemask_epi8(quote_found);
                if mask != 0 {
                    let at = sub(ptr, start_ptr);
                    let end_idx = at + (mask.trailing_zeros() as usize);
                    let scalar = std::slice::from_raw_parts(start_ptr, end_idx);
                    let scalar = Scalar::new(scalar);
                    return Ok((scalar, &haystack[end_idx + 1..]));
                }

                ptr = ptr.add(loop_size);
            }

            parse_quote_scalar_fallback(d)
        }
    }

    // from memchr: "SSE2 is avalbale on all x86_64 targets, so no CPU feature detection is necessary"
    unsafe { inner(d) }
}

#[inline]
fn split_at_scalar_fallback(d: &[u8]) -> (Scalar<'_>, &[u8]) {
    let start_ptr = d.as_ptr();
    let end_ptr = unsafe { start_ptr.add(d.len()) };

    let nind = unsafe { forward_search(start_ptr, end_ptr, is_boundary) };
    let mut ind = nind.unwrap_or(d.len());

    // To work with cases where we have "==bar" we ensure that found index is at least one
    ind = std::cmp::max(ind, 1);
    let (scalar, rest) = d.split_at(ind);
    (Scalar::new(scalar), rest)
}

#[cfg(not(target_arch = "x86_64"))]
#[inline]
fn split_at_scalar(d: &[u8]) -> (Scalar<'_>, &[u8]) {
    split_at_scalar_fallback(d)
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn split_at_scalar(d: &[u8]) -> (Scalar<'_>, &[u8]) {
    #[target_feature(enable = "sse2")]
    #[inline]
    #[allow(overflowing_literals)]
    unsafe fn inner(d: &[u8]) -> (Scalar<'_>, &[u8]) {
        unsafe {
            use core::arch::x86_64::*;
            let start_ptr = d.as_ptr();
            let loop_size = std::mem::size_of::<__m128i>();
            let end_ptr = d.as_ptr_range().end.sub(loop_size.min(d.len()));
            let mut ptr = start_ptr;

            // Here we use SIMD instructions to detect certain bytes.
            // The method used is described here:
            // http://0x80.pl/articles/simd-byte-lookup.html
            //
            // Loop partially auto-generated by the author's python code:
            // https://github.com/WojciechMula/simd-byte-lookup
            //
            // Interestingly the naive approach had the best performance.
            // To save myself some future time, here is the nibble diagram
            // of the characters that define a boundary character
            //
            //    lo / hi nibble
            //    +----------------- | ---------------
            //    | 0 1 2 3 4 5 6 7  | 8 9 a b c d e f
            //  --+----------------- | ---------------
            //  0 | . . x . . . . .  | . . . . . . . .
            //  1 | . . x . . . . .  | . . . . . . . .
            //  2 | . . . . . . . .  | . . . . . . . .
            //  3 | . . x . . . . .  | . . . . . . . .
            //  4 | . . . . . . . .  | . . . . . . . .
            //  5 | . . . . . . . .  | . . . . . . . .
            //  6 | . . . . . . . .  | . . . . . . . .
            //  7 | . . . . . . . .  | . . . . . . . .
            //  8 | . . . . . . . .  | . . . . . . . .
            //  9 | x . . . . . . .  | . . . . . . . .
            //  a | x . . . . . . .  | . . . . . . . .
            //  b | x . . . . x . x  | . . . . . . . .
            //  c | x . . x . . . .  | . . . . . . . .
            //  d | x . . x . x . x  | . . . . . . . .
            //  e | . . . x . . . .  | . . . . . . . .
            //  f | . . . . . . . .  | . . . . . . . .
            //
            // \t = 0x09
            // \n = 0x0a
            // \v = 0x0b *
            // \f = 0x0c *
            // \r = 0x0d
            // sp = 0x20
            //  ! = 0x21 *
            //  # = 0x23
            //  ; = 0x3b
            //  < = 0x3c
            //  = = 0x3d
            //  > = 0x3e
            //  [ = 0x5b
            //  ] = 0x5d
            //  { = 0x7b
            //  } = 0x7d
            // * = unknown if boundary character. Can be removed for perf
            while ptr < end_ptr {
                let input = _mm_loadu_si128(ptr as *const __m128i);
                let t0 = _mm_cmpeq_epi8(input, _mm_set1_epi8(9));
                let mut result = t0;
                let t1 = _mm_cmpeq_epi8(input, _mm_set1_epi8(10));
                result = _mm_or_si128(result, t1);
                let t2 = _mm_cmpeq_epi8(input, _mm_set1_epi8(13));
                result = _mm_or_si128(result, t2);
                let t3 = _mm_cmpeq_epi8(input, _mm_set1_epi8(32));
                result = _mm_or_si128(result, t3);
                let t4 = _mm_cmpeq_epi8(input, _mm_set1_epi8(35));
                result = _mm_or_si128(result, t4);
                let t5 = _mm_cmpeq_epi8(input, _mm_set1_epi8(59));
                result = _mm_or_si128(result, t5);
                let t6 = _mm_cmpeq_epi8(input, _mm_set1_epi8(60));
                result = _mm_or_si128(result, t6);
                let t7 = _mm_cmpeq_epi8(input, _mm_set1_epi8(61));
                result = _mm_or_si128(result, t7);
                let t8 = _mm_cmpeq_epi8(input, _mm_set1_epi8(62));
                result = _mm_or_si128(result, t8);
                let t9 = _mm_cmpeq_epi8(input, _mm_set1_epi8(123));
                result = _mm_or_si128(result, t9);
                let t10 = _mm_cmpeq_epi8(input, _mm_set1_epi8(125));
                result = _mm_or_si128(result, t10);
                let t11 = _mm_cmpeq_epi8(input, _mm_set1_epi8(91));
                result = _mm_or_si128(result, t11);
                let t12 = _mm_cmpeq_epi8(input, _mm_set1_epi8(93));
                result = _mm_or_si128(result, t12);

                let found_mask = _mm_movemask_epi8(result);
                if found_mask != 0 {
                    let at = sub(ptr, start_ptr);
                    let end_idx = at + (found_mask.trailing_zeros() as usize);
                    let end_idx = std::cmp::max(end_idx, 1);
                    let scalar = std::slice::from_raw_parts(start_ptr, end_idx);
                    let scalar = Scalar::new(scalar);
                    return (scalar, &d[end_idx..]);
                }
                ptr = ptr.add(loop_size);
            }

            split_at_scalar_fallback(d)
        }
    }

    // from memchr: "SSE2 is avalbale on all x86_64 targets, so no CPU feature detection is necessary"
    unsafe { inner(d) }
}

impl<'a> TextTape<'a> {
    /// Creates a new text tape
    pub fn new() -> Self {
        Default::default()
    }

    /// Convenience method for creating a text parser and parsing the given input
    pub fn from_slice(data: &[u8]) -> Result<TextTape<'_>, Error> {
        TextTapeParser.parse_slice(data)
    }

    /// Returns a parser for text data
    pub fn parser() -> TextTapeParser {
        TextTapeParser
    }

    /// Return the parsed tokens
    pub fn tokens(&self) -> &[TextToken<'a>] {
        self.token_tape.as_slice()
    }

    /// Return a mutable reference to the tokens for in-place modification
    ///
    /// # Safety and Correctness Caveats
    ///
    /// When modifying tokens in-place, the caller is **fully responsible** for ensuring
    /// that the resulting token stream maintains structural validity. Improper modifications
    /// can lead to undefined behavior, panics, or incorrect parsing results.
    ///
    /// ## Critical Requirements:
    ///
    /// - `Object { end: usize }` and `Array { end: usize }` tokens must have valid `end` indices
    /// - The `end` index must point to the corresponding `End(start)` token
    /// - `End(start)` tokens must have the correct `start` index pointing back to the opening token
    /// - Modifying container tokens requires updating **both** the opening and closing indices
    /// - Object keys must be scalar tokens
    ///
    /// ## Safe Modifications
    /// The following modifications are generally safe:
    /// - Changing scalar values (`Unquoted` ↔ `Quoted`)
    /// - Modifying operator types (`Operator::Exists` → `Operator::Equal`)
    /// - Updating scalar content (with same lifetime constraints)
    ///
    /// ## Example: Safe Operator Modification
    /// ```rust
    /// # use jomini::{TextTape, text::{TextToken, Operator}};
    /// let mut tape = TextTape::from_slice(b"foo ?= 10").unwrap();
    ///
    /// // Safe: Replace exists operator with equals operator
    /// for token in tape.tokens_mut() {
    ///     if let TextToken::Operator(Operator::Exists) = token {
    ///         *token = TextToken::Operator(Operator::Equal);
    ///     }
    /// }
    /// ```
    pub fn tokens_mut(&mut self) -> &mut [TextToken<'a>] {
        self.token_tape.as_mut_slice()
    }

    /// Return if there was a UTF8 BOM in the data
    pub fn utf8_bom(&self) -> bool {
        self.utf8_bom
    }
}

impl<'a> ParserState<'a, '_> {
    fn offset(&self, data: &[u8]) -> usize {
        self.original_length - data.len()
    }

    /// Skips whitespace that may terminate the file
    #[inline]
    fn skip_ws_t(&self, data: &'a [u8]) -> Option<&'a [u8]> {
        unsafe {
            let start_ptr = data.as_ptr_range().start;
            let end_ptr = data.as_ptr_range().end;
            let mut ptr = start_ptr;
            while ptr < end_ptr {
                match *ptr {
                    b' ' | b'\t' | b'\n' | b'\r' | b';' => {}
                    b'#' => loop {
                        ptr = ptr.add(1);
                        if ptr == end_ptr {
                            return None;
                        } else if *ptr == b'\n' {
                            break;
                        }
                    },
                    _ => {
                        let rest = std::slice::from_raw_parts(ptr, sub(end_ptr, ptr));
                        return Some(rest);
                    }
                }
                ptr = ptr.add(1);
            }
        }

        None
    }

    #[inline]
    fn parse_quote_scalar(&mut self, d: &'a [u8]) -> Result<&'a [u8], Error> {
        let (scalar, rest) = parse_quote_scalar(d)?;
        self.token_tape.push(TextToken::Quoted(scalar));
        Ok(rest)
    }

    #[inline(never)]
    fn parse_variable(&mut self, d: &'a [u8]) -> Result<&'a [u8], Error> {
        // detect if the variable is interpolated
        if d.get(1).is_some_and(|&x| x == b'[') {
            let mut pos = 2;
            while pos < d.len() {
                if d[pos] == b']' {
                    let (scalar, rest) = d.split_at(pos + 1);
                    let scalar = Scalar::new(scalar);
                    self.token_tape.push(TextToken::Unquoted(scalar));
                    return Ok(rest);
                } else {
                    pos += 1;
                }
            }

            Err(Error::eof())
        } else {
            let (scalar, rest) = split_at_scalar(d);
            self.token_tape.push(TextToken::Unquoted(scalar));
            Ok(rest)
        }
    }

    #[inline]
    fn parse_scalar(&mut self, d: &'a [u8]) -> &'a [u8] {
        let (scalar, rest) = split_at_scalar(d);
        self.token_tape.push(TextToken::Unquoted(scalar));
        rest
    }

    /// Clear previously parsed data and parse the given data
    #[inline]
    pub fn parse(&mut self) -> Result<(), Error> {
        let mut data = self.data;
        let mut state = ParseState::Key;

        self.utf8_bom = data.get(..3).is_some_and(|x| x == [0xef, 0xbb, 0xbf]);
        if self.utf8_bom {
            data = &data[3..];
        }

        let mut mixed_mode = false;
        let mut parent_ind = 0;
        loop {
            let d = match self.skip_ws_t(data) {
                Some(d) => d,
                None => {
                    if state != ParseState::Key {
                        return Err(Error::eof());
                    }

                    if parent_ind == 0 {
                        return Ok(());
                    } else {
                        // Support for files that don't have enough closing brackets (ugh)
                        let grand_ind = match self.token_tape.get(parent_ind) {
                            Some(TextToken::Array { end, .. }) => *end,
                            Some(TextToken::Object { end, .. }) => *end,
                            _ => 0,
                        };

                        if grand_ind == 0 {
                            let end = self.token_tape.len();
                            self.token_tape.push(TextToken::End(parent_ind));
                            self.token_tape[parent_ind] = TextToken::Object { end, mixed: false };
                            return Ok(());
                        } else {
                            return Err(Error::eof());
                        }
                    }
                }
            };

            data = d;
            match state {
                ParseState::Key => {
                    match data[0] {
                        b'}' | b']' => {
                            let saved_mixed = mixed_mode;
                            let grand_ind = match self.token_tape.get(parent_ind) {
                                Some(TextToken::Array { end, .. }) => *end,
                                Some(TextToken::Object { end, .. }) => *end,
                                _ => 0,
                            };

                            match self.token_tape.get(grand_ind) {
                                Some(TextToken::Array { mixed, .. }) => {
                                    mixed_mode = *mixed;
                                    state = ParseState::ArrayValue;
                                }
                                Some(TextToken::Object { mixed, .. }) => {
                                    mixed_mode = *mixed;
                                    state = if mixed_mode {
                                        ParseState::ArrayValue
                                    } else {
                                        ParseState::Key
                                    }
                                }
                                _ => {
                                    mixed_mode = false;
                                    state = ParseState::Key;
                                }
                            };

                            let end_idx = self.token_tape.len();
                            if parent_ind == 0 && grand_ind == 0 {
                                // Allow extraneous close braces to support malformatted game files (ugh)
                                data = &data[1..];
                                continue;
                            }

                            self.token_tape.push(TextToken::End(parent_ind));
                            self.token_tape[parent_ind] = TextToken::Object {
                                end: end_idx,
                                mixed: saved_mixed,
                            };
                            parent_ind = grand_ind;
                            data = &data[1..];
                        }

                        // Empty object or token header
                        b'{' => {
                            data = self.skip_ws_t(&data[1..]).ok_or_else(Error::eof)?;
                            if data[0] == b'}' {
                                data = &data[1..];
                                continue;
                            }

                            if let Some(last) = self.token_tape.last_mut()
                                && let TextToken::Unquoted(header) = last
                            {
                                *last = TextToken::Header(*header);
                                self.token_tape.push(TextToken::Array {
                                    end: 0,
                                    mixed: false,
                                });
                                state = ParseState::ParseOpen;
                                continue;
                            }

                            return Err(Error::new(ErrorKind::InvalidSyntax {
                                offset: self.offset(data),
                                msg: String::from("invalid syntax for token headers"),
                            }));
                        }

                        b'[' => {
                            data = self.parse_parameter_definition(
                                data,
                                &mut parent_ind,
                                &mut state,
                                false,
                            )?;
                        }

                        b'"' => {
                            data = self.parse_quote_scalar(data)?;
                            state = ParseState::KeyValueSeparator;
                        }

                        b'@' => {
                            data = self.parse_variable(data)?;
                            state = ParseState::KeyValueSeparator;
                        }

                        _ => {
                            data = self.parse_scalar(data);
                            state = ParseState::KeyValueSeparator;
                        }
                    }
                }
                ParseState::KeyValueSeparator => match data {
                    [b'<', b'=', ..] => {
                        self.token_tape
                            .push(TextToken::Operator(Operator::LessThanEqual));
                        data = &data[2..];
                        state = ParseState::ObjectValue;
                    }
                    [b'<', ..] => {
                        self.token_tape
                            .push(TextToken::Operator(Operator::LessThan));
                        data = &data[1..];
                        state = ParseState::ObjectValue;
                    }
                    [b'>', b'=', ..] => {
                        self.token_tape
                            .push(TextToken::Operator(Operator::GreaterThanEqual));
                        data = &data[2..];
                        state = ParseState::ObjectValue;
                    }
                    [b'>', ..] => {
                        self.token_tape
                            .push(TextToken::Operator(Operator::GreaterThan));
                        data = &data[1..];
                        state = ParseState::ObjectValue;
                    }
                    [b'!', b'=', ..] => {
                        self.token_tape
                            .push(TextToken::Operator(Operator::NotEqual));
                        data = &data[2..];
                        state = ParseState::ObjectValue;
                    }
                    [b'?', b'=', ..] => {
                        self.token_tape.push(TextToken::Operator(Operator::Exists));
                        data = &data[2..];
                        state = ParseState::ObjectValue;
                    }
                    [b'=', b'=', ..] => {
                        self.token_tape.push(TextToken::Operator(Operator::Exact));
                        data = &data[2..];
                        state = ParseState::ObjectValue;
                    }
                    [b'=', ..] if mixed_mode => {
                        self.token_tape.push(TextToken::Operator(Operator::Equal));
                        data = &data[1..];
                    }
                    [b'=', ..] => {
                        data = &data[1..];
                        state = ParseState::ObjectValue;
                    }
                    [b'{', ..] => {
                        state = ParseState::ObjectValue;
                    }
                    [b'}', ..] => {
                        self.token_tape
                            .insert(self.token_tape.len() - 1, TextToken::MixedContainer);
                        state = ParseState::ArrayValue;
                        mixed_mode = true;
                    }
                    _ => {
                        self.token_tape
                            .insert(self.token_tape.len() - 1, TextToken::MixedContainer);
                        state = ParseState::ArrayValue;
                        mixed_mode = true;
                    }
                },
                ParseState::ObjectValue => match data[0] {
                    b'{' => {
                        self.token_tape.push(TextToken::Array {
                            end: 0,
                            mixed: false,
                        });
                        state = ParseState::ParseOpen;
                        data = &data[1..];
                    }

                    b'}' => {
                        return Err(Error::new(ErrorKind::InvalidSyntax {
                            msg: String::from("encountered '}' for object value"),
                            offset: self.offset(data),
                        }));
                    }

                    b'"' => {
                        data = self.parse_quote_scalar(data)?;
                        state = ParseState::Key;
                    }
                    b'@' => {
                        data = self.parse_variable(data)?;
                        state = ParseState::Key;
                    }
                    _ => {
                        data = self.parse_scalar(data);
                        state = ParseState::Key;
                    }
                },
                ParseState::ParseOpen => {
                    match data[0] {
                        // Empty array
                        b'}' => {
                            let ind = self.token_tape.len() - 1;

                            match self.token_tape.get(parent_ind) {
                                Some(TextToken::Array { mixed, .. }) => {
                                    mixed_mode = *mixed;
                                    state = ParseState::ArrayValue;
                                }
                                Some(TextToken::Object { mixed, .. }) => {
                                    mixed_mode = *mixed;
                                    state = if mixed_mode {
                                        ParseState::ArrayValue
                                    } else {
                                        ParseState::Key
                                    }
                                }
                                _ => {
                                    mixed_mode = false;
                                    state = ParseState::Key;
                                }
                            };

                            self.token_tape[ind] = TextToken::Array {
                                end: ind + 1,
                                mixed: false,
                            };
                            self.token_tape.push(TextToken::End(ind));
                            data = &data[1..];
                            continue;
                        }

                        // start of a parameter definition
                        b'[' => {
                            if mixed_mode {
                                return Err(Error::new(ErrorKind::InvalidSyntax {
                                    msg: String::from(
                                        "mixed object and array container not expected",
                                    ),
                                    offset: self.offset(data),
                                }));
                            }

                            data = self.parse_parameter_definition(
                                data,
                                &mut parent_ind,
                                &mut state,
                                true,
                            )?;
                            continue;
                        }

                        // array of objects, another array
                        b'{' => {
                            let scratch = self.skip_ws_t(&data[1..]).ok_or_else(Error::eof)?;
                            if scratch[0] == b'}' {
                                data = &scratch[1..];
                                continue;
                            }

                            mixed_mode = false;
                            let ind = self.token_tape.len() - 1;
                            self.token_tape[ind] = TextToken::Array {
                                end: parent_ind,
                                mixed: mixed_mode,
                            };
                            parent_ind = ind;
                            state = ParseState::ArrayValue;
                            continue;
                        }
                        b'"' => {
                            data = self.parse_quote_scalar(data)?;
                        }
                        b'@' => {
                            data = self.parse_variable(data)?;
                        }
                        _ => {
                            data = self.parse_scalar(data);
                        }
                    }

                    if mixed_mode
                        && let Some(
                            TextToken::Array { mixed, .. } | TextToken::Object { mixed, .. },
                        ) = self.token_tape.get_mut(parent_ind)
                    {
                        *mixed = mixed_mode;
                    }

                    mixed_mode = false;
                    data = self.skip_ws_t(data).ok_or_else(Error::eof)?;
                    match data[0] {
                        b'=' | b'>' | b'<' | b'?' | b'!' => {
                            let ind = self.token_tape.len() - 2;
                            self.token_tape[ind] = TextToken::Object {
                                end: parent_ind,
                                mixed: mixed_mode,
                            };
                            parent_ind = ind;
                            state = ParseState::KeyValueSeparator;
                        }
                        _ => {
                            let ind = self.token_tape.len() - 2;
                            self.token_tape[ind] = TextToken::Array {
                                end: parent_ind,
                                mixed: mixed_mode,
                            };
                            parent_ind = ind;
                            state = ParseState::ArrayValue;
                        }
                    }
                }
                ParseState::ArrayValue => match data[0] {
                    b'{' => {
                        self.token_tape.push(TextToken::Array {
                            end: 0,
                            mixed: false,
                        });
                        state = ParseState::ParseOpen;
                        data = &data[1..];
                    }
                    b'}' => {
                        let saved_mixed_mode = mixed_mode;
                        let (grand_ind, is_array) = match self.token_tape.get(parent_ind) {
                            Some(TextToken::Array { end, .. }) => (*end, true),
                            Some(TextToken::Object { end, .. }) => (*end, false),
                            _ => (0, false),
                        };

                        match self.token_tape.get(grand_ind) {
                            Some(TextToken::Array { mixed, .. }) => {
                                mixed_mode = *mixed;
                                state = ParseState::ArrayValue;
                            }
                            Some(TextToken::Object { mixed, .. }) => {
                                mixed_mode = *mixed;
                                state = if mixed_mode {
                                    ParseState::ArrayValue
                                } else {
                                    ParseState::Key
                                }
                            }
                            _ => {
                                mixed_mode = false;
                                state = ParseState::Key;
                            }
                        };

                        if parent_ind == 0 && grand_ind == 0 {
                            return Err(Error::new(ErrorKind::StackEmpty {
                                offset: self.offset(data),
                            }));
                        }

                        let end_idx = self.token_tape.len();
                        self.token_tape[parent_ind] = if is_array {
                            TextToken::Array {
                                end: end_idx,
                                mixed: saved_mixed_mode,
                            }
                        } else {
                            TextToken::Object {
                                end: end_idx,
                                mixed: saved_mixed_mode,
                            }
                        };

                        self.token_tape.push(TextToken::End(parent_ind));
                        parent_ind = grand_ind;
                        data = &data[1..];
                    }
                    b'"' => {
                        data = self.parse_quote_scalar(data)?;
                        state = ParseState::ArrayValue;
                    }
                    b'@' => {
                        data = self.parse_variable(data)?;
                        state = ParseState::ArrayValue;
                    }
                    b'<' | b'>' | b'!' | b'=' => {
                        if !mixed_mode {
                            let can_precede_operator = match self.token_tape.last() {
                                // Regular scalars can precede operators
                                Some(token) if token.as_scalar().is_some() => true,
                                // End tokens that close objects or arrays in object template syntax
                                // For object template syntax, we should just continue parsing without creating operators
                                Some(TextToken::End(start_idx)) => {
                                    if matches!(
                                        self.token_tape.get(*start_idx),
                                        Some(TextToken::Object { .. })
                                            | Some(TextToken::Array { .. })
                                    ) {
                                        // This is object template syntax, don't treat as operator, continue parsing value
                                        if data[0] == b'=' {
                                            data = &data[1..];
                                            continue;
                                        }
                                    }
                                    false
                                }
                                _ => false,
                            };

                            if can_precede_operator {
                                self.token_tape
                                    .insert(self.token_tape.len() - 1, TextToken::MixedContainer);
                                mixed_mode = true;
                            } else {
                                return Err(Error::new(ErrorKind::InvalidSyntax {
                                    msg: String::from("expected a scalar to precede an operator"),
                                    offset: self.offset(data) - 1,
                                }));
                            }
                        }

                        match data {
                            [b'<', b'=', ..] => {
                                self.token_tape
                                    .push(TextToken::Operator(Operator::LessThanEqual));
                                data = &data[2..];
                            }
                            [b'<', ..] => {
                                self.token_tape
                                    .push(TextToken::Operator(Operator::LessThan));
                                data = &data[1..];
                            }
                            [b'>', b'=', ..] => {
                                self.token_tape
                                    .push(TextToken::Operator(Operator::GreaterThanEqual));
                                data = &data[2..];
                            }
                            [b'>', ..] => {
                                self.token_tape
                                    .push(TextToken::Operator(Operator::GreaterThan));
                                data = &data[1..];
                            }
                            [b'!', b'=', ..] => {
                                self.token_tape
                                    .push(TextToken::Operator(Operator::NotEqual));
                                data = &data[2..];
                            }
                            [b'=', b'=', ..] => {
                                self.token_tape.push(TextToken::Operator(Operator::Exact));
                                data = &data[2..];
                            }
                            [b'=', ..] => {
                                self.token_tape.push(TextToken::Operator(Operator::Equal));
                                data = &data[1..];
                            }
                            _ => {
                                return Err(Error::new(ErrorKind::InvalidSyntax {
                                    msg: String::from("unrecognized operator"),
                                    offset: self.offset(data) - 1,
                                }));
                            }
                        }
                    }
                    _ => {
                        data = self.parse_scalar(data);
                        state = ParseState::ArrayValue;
                    }
                },
            }
        }
    }

    fn parse_parameter_definition(
        &mut self,
        data: &'a [u8],
        parent_ind: &mut usize,
        state: &mut ParseState,
        initial: bool,
    ) -> Result<&'a [u8], Error> {
        if !matches!(data.get(1), Some(&x) if x == b'[') {
            return Err(Error::new(ErrorKind::InvalidSyntax {
                offset: self.offset(data),
                msg: String::from("expected start of parameter definition"),
            }));
        }

        // This is for parse_open to know and we signal that a parameter
        // definition means an object as parameters should be uniquely named
        if initial {
            let ind = self.token_tape.len() - 1;
            self.token_tape[ind] = TextToken::Object {
                end: *parent_ind,
                mixed: false,
            };
            *parent_ind = ind;
        }

        let is_undefined = matches!(data.get(2), Some(&x) if x == b'!');
        let data = data
            .get(2 + is_undefined as usize..)
            .ok_or_else(Error::eof)?;

        if data.is_empty() {
            return Err(Error::eof());
        }

        let (scalar, data) = split_at_scalar(data);
        if !matches!(data.first(), Some(&x) if x == b']') {
            return Err(Error::new(ErrorKind::InvalidSyntax {
                offset: self.offset(data),
                msg: String::from("expected end of parameter name"),
            }));
        }

        let data = &data[1..];

        let token = if is_undefined {
            TextToken::UndefinedParameter(scalar)
        } else {
            TextToken::Parameter(scalar)
        };

        self.token_tape.push(token);

        // now we have to determine if are looking at a parameter value or
        // parameter object. We know when we are looking at a parameter value
        // when `]` is encountered first after the value else it's a key
        let data = self.skip_ws_t(data).ok_or_else(Error::eof)?;
        let (key_or_value, data) = split_at_scalar(data);
        let data = self.skip_ws_t(data).ok_or_else(Error::eof)?;
        if data[0] == b']' {
            self.token_tape.push(TextToken::Unquoted(key_or_value));
            *state = ParseState::Key;
            Ok(&data[1..])
        } else {
            let grand_ind = *parent_ind;
            *parent_ind = self.token_tape.len();
            self.token_tape.push(TextToken::Object {
                end: grand_ind,
                mixed: false,
            });
            self.token_tape.push(TextToken::Unquoted(key_or_value));
            *state = ParseState::KeyValueSeparator;
            Ok(data)
        }
    }
}

fn sub(a: *const u8, b: *const u8) -> usize {
    debug_assert!(a >= b);
    (a as usize) - (b as usize)
}

#[inline(always)]
unsafe fn forward_search<F: Fn(u8) -> bool>(
    start_ptr: *const u8,
    end_ptr: *const u8,
    confirm: F,
) -> Option<usize> {
    unsafe {
        let mut ptr = start_ptr;
        while ptr < end_ptr {
            if confirm(*ptr) {
                return Some(sub(ptr, start_ptr));
            }
            ptr = ptr.offset(1);
        }

        None
    }
}

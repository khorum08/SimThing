// Vendored from github.com/rakaly/jomini @ v0.34.1 (commit fff00d8c7f8f06c084d776d1a2c98b34324e64ed)
// License: MIT - see crates/simthing-clausething/src/jomini/LICENSE
use super::fnv::FnvBuildHasher;
use crate::jomini::{
    DeserializeError, DeserializeErrorKind, Encoding, Scalar, TextTape, TextToken, text::Operator,
};
use std::{
    borrow::Cow,
    collections::{HashMap, hash_map::Entry},
};

pub type KeyValue<'data, 'tokens, E> = (
    ScalarReader<'data, E>,
    Option<Operator>,
    ValueReader<'data, 'tokens, E>,
);

pub type KeyValues<'data, 'tokens, E> = (ScalarReader<'data, E>, GroupEntry<'data, 'tokens, E>);

/// Calculate what index the next value is. This assumes that a header + value
/// are two separate values
#[inline]
fn next_idx_header(tokens: &[TextToken], idx: usize) -> usize {
    match tokens[idx] {
        TextToken::Array { end, .. } | TextToken::Object { end, .. } => end + 1,
        TextToken::Operator(_) | TextToken::MixedContainer => idx + 2,
        _ => idx + 1,
    }
}

/// Calculate what index the next value is. This assumes that a header + value
/// is one value
#[inline]
fn next_idx(tokens: &[TextToken], idx: usize) -> usize {
    match tokens[idx] {
        TextToken::Array { end, .. } | TextToken::Object { end, .. } => end + 1,
        TextToken::Operator(_) => next_idx(tokens, idx + 1),
        TextToken::Header(_) => next_idx_header(tokens, idx + 1),
        _ => idx + 1,
    }
}

#[inline]
fn next_idx_values(tokens: &[TextToken], idx: usize) -> usize {
    match tokens[idx] {
        TextToken::Array { end, .. } | TextToken::Object { end, .. } => end + 1,
        _ => idx + 1,
    }
}

#[inline]
fn fields_len(tokens: &[TextToken], start_ind: usize, end_ind: usize) -> usize {
    let mut ind = start_ind;
    let mut count = 0;
    while ind < end_ind {
        let key_ind = ind;
        if tokens[key_ind] == TextToken::MixedContainer {
            return count;
        }

        let value_ind = match tokens[key_ind + 1] {
            TextToken::Operator(_) => key_ind + 2,
            _ => key_ind + 1,
        };
        ind = next_idx(tokens, value_ind);
        count += 1;
    }

    count
}

#[inline]
pub fn values_len(tokens: &[TextToken], start_ind: usize, end_ind: usize) -> usize {
    let mut count = 0;
    let mut ind = start_ind;
    while ind < end_ind {
        ind = next_idx_values(tokens, ind);
        count += 1;
    }

    count
}

type OpValue<'data, 'tokens, E> = (Option<Operator>, ValueReader<'data, 'tokens, E>);

/// Iterator over values grouped by duplicate keys
///
/// See [FieldGroupsIter](crate::jomini::text::FieldGroupsIter) for a worked example
pub struct GroupEntryIter<'data, 'tokens, 'parent, E> {
    index: usize,
    parent: &'parent GroupEntry<'data, 'tokens, E>,
}

impl<'data, 'tokens, E> Iterator for GroupEntryIter<'data, 'tokens, '_, E>
where
    E: Clone,
{
    type Item = (Option<Operator>, ValueReader<'data, 'tokens, E>);

    fn next(&mut self) -> Option<Self::Item> {
        match &self.parent {
            GroupEntry::One((op, val)) => {
                if self.index == 0 {
                    self.index += 1;
                    Some((*op, (*val).clone()))
                } else {
                    None
                }
            }
            GroupEntry::Multiple(entries) => {
                let result = entries.get(self.index);
                self.index += 1;
                result.map(|(op, val)| (*op, (*val).clone()))
            }
        }
    }
}

/// Represents a group of values for duplicate keys
///
/// May contain one or many values
///
/// ```
/// use jomini::TextTape;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tape = TextTape::from_slice(b"name=a core=b core=c")?;
/// let reader = tape.windows1252_reader();
/// let mut fields = reader.field_groups();
/// let first_group = fields.next();
/// let first_key = first_group.as_ref().map(|(key, _)| key.read_str());
/// assert_eq!(first_key.as_deref(), Some("name"));
/// let first_values_len = first_group.as_ref().map(|(_, group)| group.len());
/// assert_eq!(first_values_len, Some(1));
/// let first_values = first_group.map(|(_, group)| {
///     group.values()
///         .filter_map(|(_op, val)| val.read_string().ok())
///         .collect()
/// });
/// assert_eq!(first_values, Some(vec![String::from("a")]));
///
/// let second_group = fields.next();
/// let second_key = second_group.as_ref().map(|(key, _)| key.read_str());
/// assert_eq!(second_key.as_deref(), Some("core"));
/// let second_values = second_group.as_ref().map(|(_, group)| group.len());
/// assert_eq!(second_values, Some(2));
/// let second_values = second_group.map(|(_, group)| {
///     group.values()
///         .filter_map(|(_op, val)| val.read_string().ok())
///         .collect()
/// });
/// assert_eq!(second_values, Some(vec![String::from("b"), String::from("c")]));
/// # Ok(())
/// # }
/// ```
pub enum GroupEntry<'data, 'tokens, E> {
    /// Represents that the group is composed of only one value
    ///
    /// Most fields should only occur once, so this variant is optimized to
    /// not require a memory allocation (unlike the `Multiple` variant).
    One(OpValue<'data, 'tokens, E>),

    /// Represents that the group is composed of several values
    Multiple(Vec<OpValue<'data, 'tokens, E>>),
}

impl<'data, 'tokens, E> GroupEntry<'data, 'tokens, E> {
    /// Returns an iterator that includes all the values
    pub fn values<'parent>(&'parent self) -> GroupEntryIter<'data, 'tokens, 'parent, E> {
        GroupEntryIter {
            index: 0,
            parent: self,
        }
    }

    /// A group can never be empty so this returns false
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns the number of values in the group
    pub fn len(&self) -> usize {
        match &self {
            GroupEntry::One(_) => 1,
            GroupEntry::Multiple(x) => x.len(),
        }
    }
}

/// All possible text reader variants
#[derive(Debug, Clone)]
pub enum Reader<'data, 'tokens, E> {
    /// object reader
    Object(ObjectReader<'data, 'tokens, E>),

    /// array reader
    Array(ArrayReader<'data, 'tokens, E>),

    /// scalar reader
    Scalar(ScalarReader<'data, E>),

    /// value reader
    Value(ValueReader<'data, 'tokens, E>),
}

impl<'data, E> Reader<'data, '_, E>
where
    E: Encoding + Clone,
{
    /// Interpret value as a string
    #[inline]
    pub fn read_str(&self) -> Result<Cow<'data, str>, DeserializeError> {
        match &self {
            Reader::Scalar(x) => Ok(x.read_str()),
            Reader::Value(x) => x.read_str(),
            _ => Err(DeserializeError {
                kind: DeserializeErrorKind::Unsupported("not a scalar"),
            }),
        }
    }

    /// Interpret value as a string
    #[inline]
    pub fn read_string(&self) -> Result<String, DeserializeError> {
        match &self {
            Reader::Scalar(x) => Ok(x.read_string()),
            Reader::Value(x) => x.read_string(),
            _ => Err(DeserializeError {
                kind: DeserializeErrorKind::Unsupported("not a scalar"),
            }),
        }
    }

    /// Interpret value as a scalar
    #[inline]
    pub fn read_scalar(&self) -> Result<Scalar<'data>, DeserializeError> {
        match &self {
            Reader::Scalar(x) => Ok(x.read_scalar()),
            Reader::Value(x) => x.read_scalar(),
            _ => Err(DeserializeError {
                kind: DeserializeErrorKind::Unsupported("not a scalar"),
            }),
        }
    }
}

/// Iterator over fields of an object grouped by key
///
/// Since objects can have duplicated keys across fields, this iterator
/// consolidates them such that all values with the same key are grouped
/// together in the order that they appear in the object. Key order is
/// also equivalent, except that already seen keys will be skipped, as
/// those values have already been seen in an earlier group.
///
/// The process of grouping values together is more expensive than simply
/// iterating the keys in order, so when possible prefer
/// [`ObjectReader::fields()`](crate::jomini::text::ObjectReader::fields) over
/// [`ObjectReader::field_groups()`](crate::jomini::text::ObjectReader::field_groups).
///
/// These groups can be easily iterated:
///
/// ```
/// use jomini::TextTape;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tape = TextTape::from_slice(b"name=a core=b core=c")?;
/// let reader = tape.windows1252_reader();
/// for (key, group) in reader.field_groups() {
///     match key.read_str().as_ref() {
///         "name" => assert_eq!(group.len(), 1),
///         "core" => assert_eq!(group.len(), 2),
///         x => panic!("unexpected key: {}", x),
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
/// And picked apart:
///
/// ```
/// use jomini::TextTape;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tape = TextTape::from_slice(b"name=a core=b core=c")?;
/// let reader = tape.windows1252_reader();
/// let mut fields = reader.field_groups();
/// let first_group = fields.next();
/// let first_key = first_group.as_ref().map(|(key, _)| key.read_str());
/// assert_eq!(first_key.as_deref(), Some("name"));
/// let first_values_len = first_group.as_ref().map(|(_, group)| group.len());
/// assert_eq!(first_values_len, Some(1));
/// let first_values = first_group.map(|(_, group)| {
///     group.values()
///         .filter_map(|(_op, val)| val.read_string().ok())
///         .collect()
/// });
/// assert_eq!(first_values, Some(vec![String::from("a")]));
///
/// let second_group = fields.next();
/// let second_key = second_group.as_ref().map(|(key, _)| key.read_str());
/// assert_eq!(second_key.as_deref(), Some("core"));
/// let second_values = second_group.as_ref().map(|(_, group)| group.len());
/// assert_eq!(second_values, Some(2));
/// let second_values = second_group.map(|(_, group)| {
///     group.values()
///         .filter_map(|(_op, val)| val.read_string().ok())
///         .collect()
/// });
/// assert_eq!(second_values, Some(vec![String::from("b"), String::from("c")]));
/// # Ok(())
/// # }
/// ```
pub struct FieldGroupsIter<'data, 'tokens, E> {
    key_indices: HashMap<&'data [u8], Vec<OpValue<'data, 'tokens, E>>, FnvBuildHasher>,
    fields: FieldsIter<'data, 'tokens, E>,
}

impl<'data, 'tokens, E> FieldGroupsIter<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    fn new(reader: &ObjectReader<'data, 'tokens, E>) -> Self {
        // Using the fnv hasher improved throughput of the eu4 json benchmark
        // by over 15%.
        let mut key_indices =
            HashMap::with_capacity_and_hasher(reader.fields_len(), FnvBuildHasher::default());
        for (key, op, val) in reader.fields() {
            let entry = key_indices.entry(key.read_scalar().as_bytes());

            match entry {
                Entry::Vacant(x) => {
                    x.insert(Vec::with_capacity(0));
                }
                Entry::Occupied(mut x) => {
                    x.get_mut().push((op, val));
                }
            }
        }

        let fields = reader.fields();

        FieldGroupsIter {
            key_indices,
            fields,
        }
    }

    /// See [the other `remainder` documentation](crate::jomini::text::FieldsIter::remainder)
    pub fn remainder(&self) -> ArrayReader<'data, 'tokens, E> {
        self.fields.remainder()
    }
}

impl<'data, 'tokens, E> Iterator for FieldGroupsIter<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    type Item = KeyValues<'data, 'tokens, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (key, op, value) = self.fields.next()?;

            if let Some((_key, mut entries)) =
                self.key_indices.remove_entry(key.read_scalar().as_bytes())
            {
                if entries.is_empty() {
                    return Some((key, GroupEntry::One((op, value))));
                } else {
                    entries.insert(0, (op, value));
                    return Some((key, GroupEntry::Multiple(entries)));
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.key_indices.len(), None)
    }
}

/// Iterator over fields of an object in the order that they appear
///
/// Since objects can have duplicated keys across fields, this iterator
/// may yield items that have duplicate keys.
///
/// Fields can be easily iterated:
///
/// ```
/// use jomini::TextTape;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tape = TextTape::from_slice(b"name=a core=b core=c")?;
/// let reader = tape.windows1252_reader();
/// let (names, cores) = reader
///     .fields()
///     .fold((0, 0), |(names, cores), (key, _op, _value)| {
///         match key.read_str().as_ref() {
///             "name" => (names + 1, cores),
///             "core" => (names, cores + 1),
///             x => panic!("unexpected key: {}", x),
///         }
///     });
/// assert_eq!((1, 2), (names, cores));
/// # Ok(())
/// # }
/// ```
///
/// And picked apart:
///
/// ```
/// use jomini::TextTape;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tape = TextTape::from_slice(b"name=a core=b core=c")?;
/// let reader = tape.windows1252_reader();
/// let mut fields = reader.fields();
/// let (first_key, _op, first_val) = fields.next().unwrap();
/// assert_eq!(first_key.read_str(), "name");
/// assert_eq!(first_val.read_str().ok().as_deref(), Some("a"));
/// # Ok(())
/// # }
/// ```
pub struct FieldsIter<'data, 'tokens, E> {
    start_ind: usize,
    token_ind: usize,
    end_ind: usize,
    tokens: &'tokens [TextToken<'data>],
    encoding: E,
}

impl<'data, 'tokens, E> FieldsIter<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    fn new(reader: &ObjectReader<'data, 'tokens, E>) -> Self {
        FieldsIter {
            start_ind: reader.start_ind.saturating_sub(1),
            token_ind: reader.start_ind,
            end_ind: reader.end_ind,
            tokens: reader.tokens,
            encoding: reader.encoding.clone(),
        }
    }

    /// Returns the remaining values from an object if the container is an
    /// object that transitions into an array.
    pub fn remainder(&self) -> ArrayReader<'data, 'tokens, E> {
        let start = self
            .tokens
            .get(self.token_ind)
            .map(|x| match x {
                TextToken::MixedContainer => self.token_ind + 1,
                TextToken::End(y) => {
                    if let Some(TextToken::Array { .. }) = self.tokens.get(*y) {
                        *y + 1
                    } else {
                        self.token_ind
                    }
                }
                _ => self.token_ind,
            })
            .unwrap_or(self.end_ind);

        ArrayReader {
            start_ind: start,
            end_ind: self.end_ind,
            encoding: self.encoding.clone(),
            tokens: self.tokens,
        }
    }
}

impl<'data, 'tokens, E> Iterator for FieldsIter<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    type Item = KeyValue<'data, 'tokens, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.token_ind >= self.end_ind {
            return None;
        }

        let key_ind = self.token_ind;
        let token = self.tokens[key_ind].clone();
        let key_scalar = match token {
            TextToken::Quoted(x)
            | TextToken::Unquoted(x)
            | TextToken::Parameter(x)
            | TextToken::UndefinedParameter(x) => x,
            TextToken::MixedContainer => {
                return None;
            }
            _ => {
                // this is a broken invariant, so we safely recover by saying the object
                // has no more fields
                debug_assert!(false, "All keys should be scalars, not {:?}", &token);
                return None;
            }
        };

        let key_reader = ScalarReader {
            scalar: key_scalar,
            token,
            encoding: self.encoding.clone(),
        };

        let (op, value_ind) = match self.tokens[key_ind + 1] {
            TextToken::Operator(x) => (Some(x), key_ind + 2),
            _ => (None, key_ind + 1),
        };

        let value_reader = ValueReader {
            parent_ind: self.start_ind,
            value_ind,
            tokens: self.tokens,
            encoding: self.encoding.clone(),
        };
        self.token_ind = next_idx(self.tokens, value_ind);
        Some((key_reader, op, value_reader))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = fields_len(self.tokens, self.token_ind, self.end_ind);
        (len, None)
    }
}

/// A reader for objects
#[derive(Debug, Clone)]
pub struct ObjectReader<'data, 'tokens, E> {
    start_ind: usize,
    end_ind: usize,
    tokens: &'tokens [TextToken<'data>],
    encoding: E,
}

impl<'data, 'tokens, E> ObjectReader<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    /// Create a new object reader from parsed data with encoded strings
    pub fn new(tape: &'tokens TextTape<'data>, encoding: E) -> Self {
        let tokens = tape.tokens();
        ObjectReader {
            tokens,
            end_ind: tokens.len(),
            start_ind: 0,
            encoding,
        }
    }

    /// Create a new object reader directly from a token slice
    pub fn from_tokens(tokens: &'tokens [TextToken<'data>], encoding: E) -> Self {
        ObjectReader {
            tokens,
            end_ind: tokens.len(),
            start_ind: 0,
            encoding,
        }
    }

    /// Return the number of tokens contained within the object
    ///
    /// ```
    /// use jomini::TextTape;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let tape = TextTape::from_slice(b"obj={1} foo=bar")?;
    /// let reader = tape.windows1252_reader();
    /// assert_eq!(reader.tokens_len(), 6);
    /// # Ok(())
    /// # }
    /// ```
    pub fn tokens_len(&self) -> usize {
        self.end_ind - self.start_ind
    }

    /// Deserialize from the object reader
    ///
    /// ```
    /// use jomini::TextTape;
    /// use serde::Deserialize;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #[derive(Debug, Clone, Deserialize, PartialEq)]
    /// pub struct Obj {
    ///   foo: String,
    /// }
    ///
    /// let tape = TextTape::from_slice(b"obj={foo=bar}")?;
    /// let reader = tape.windows1252_reader();
    /// let mut fields = reader.fields();
    /// let (_, _, obj_value) = fields.next().unwrap();
    /// let obj_reader = obj_value.read_object().unwrap();
    /// let result: Obj = obj_reader.deserialize().unwrap();
    /// assert_eq!(result, Obj { foo: "bar".to_string() });
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "serde")]
    pub fn deserialize<T>(&self) -> Result<T, crate::jomini::Error>
    where
        T: serde::Deserialize<'data>,
    {
        T::deserialize(&crate::jomini::TextDeserializer::from_reader(self))
    }

    /// Return the number of key value pairs that the object contains.
    pub fn fields_len(&self) -> usize {
        fields_len(self.tokens, self.start_ind, self.end_ind)
    }

    /// Iterator over fields as they appear in the object
    ///
    /// See [FieldsIter](crate::jomini::text::FieldsIter) for a worked example
    #[inline]
    pub fn fields(&self) -> FieldsIter<'data, 'tokens, E> {
        FieldsIter::new(self)
    }

    /// Iterator over fields that are grouped by key
    ///
    /// See [FieldGroupsIter](crate::jomini::text::FieldGroupsIter) for a worked example
    #[inline]
    pub fn field_groups(&self) -> FieldGroupsIter<'data, 'tokens, E> {
        FieldGroupsIter::new(self)
    }
}

/// A text reader that wraps an underlying scalar value
#[derive(Debug, Clone)]
pub struct ScalarReader<'data, E> {
    scalar: Scalar<'data>,
    token: TextToken<'data>,
    encoding: E,
}

impl<'data, E> ScalarReader<'data, E>
where
    E: Encoding,
{
    /// Decode the data with a given string encoding
    #[inline]
    pub fn read_str(&self) -> Cow<'data, str> {
        self.encoding.decode(self.scalar.as_bytes())
    }

    /// Decode the data with a given string encoding
    #[inline]
    pub fn read_string(&self) -> String {
        self.encoding.decode(self.scalar.as_bytes()).into_owned()
    }

    /// Return the underlying scalar
    #[inline]
    pub fn read_scalar(&self) -> Scalar<'data> {
        self.scalar
    }

    /// Return the token that the reader is abstracting
    #[inline]
    pub fn token(&self) -> &TextToken<'data> {
        &self.token
    }
}

/// A text reader for a text value
#[derive(Debug, Clone)]
pub struct ValueReader<'data, 'tokens, E> {
    parent_ind: usize,
    value_ind: usize,
    tokens: &'tokens [TextToken<'data>],
    encoding: E,
}

impl<'data, E> ValueReader<'data, '_, E> {
    /// Index of this value's token in the parent tape (CT-0b span metadata).
    #[inline]
    pub fn value_index(&self) -> usize {
        self.value_ind
    }

    /// Return the token that the reader is abstracting
    #[inline]
    pub fn token(&self) -> &TextToken<'data> {
        &self.tokens[self.value_ind]
    }

    #[cfg(feature = "serde")]
    pub(crate) fn next(&mut self) -> Option<&TextToken<'data>> {
        self.value_ind += 1;
        self.tokens.get(self.value_ind)
    }
}

impl<E> Encoding for ValueReader<'_, '_, E>
where
    E: Encoding,
{
    #[inline]
    fn decode<'a>(&self, data: &'a [u8]) -> Cow<'a, str> {
        self.encoding.decode(data)
    }
}

impl<'data, 'tokens, E> ValueReader<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    fn raw_str(&self) -> Option<Cow<'data, str>> {
        match self.tokens[self.value_ind] {
            TextToken::Header(s)
            | TextToken::Unquoted(s)
            | TextToken::Quoted(s)
            | TextToken::Parameter(s)
            | TextToken::UndefinedParameter(s) => Some(self.encoding.decode(s.as_bytes())),
            TextToken::Operator(s) => Some(Cow::Borrowed(s.symbol())),
            _ => None,
        }
    }

    /// Interpret the current value as string
    #[inline]
    pub fn read_str(&self) -> Result<Cow<'data, str>, DeserializeError> {
        self.raw_str().ok_or(DeserializeError {
            kind: DeserializeErrorKind::Unsupported("not a string"),
        })
    }

    /// Interpret the current value as string
    #[inline]
    pub fn read_string(&self) -> Result<String, DeserializeError> {
        self.raw_str().map(String::from).ok_or(DeserializeError {
            kind: DeserializeErrorKind::Unsupported("not a string"),
        })
    }

    /// Interpret the current value as a scalar
    #[inline]
    pub fn read_scalar(&self) -> Result<Scalar<'data>, DeserializeError> {
        self.tokens[self.value_ind]
            .as_scalar()
            .ok_or(DeserializeError {
                kind: DeserializeErrorKind::Unsupported("not a scalar"),
            })
    }

    /// Interpret the current value as an object
    #[inline]
    pub fn read_object(&self) -> Result<ObjectReader<'data, 'tokens, E>, DeserializeError> {
        match self.tokens[self.value_ind] {
            TextToken::Object { end, .. } => Ok(ObjectReader {
                tokens: self.tokens,
                start_ind: self.value_ind + 1,
                end_ind: end,
                encoding: self.encoding.clone(),
            }),

            TextToken::Array { end, .. } => Ok(ObjectReader {
                tokens: self.tokens,
                start_ind: end,
                end_ind: end,
                encoding: self.encoding.clone(),
            }),

            TextToken::MixedContainer => Ok(ObjectReader {
                tokens: self.tokens,
                start_ind: self.value_ind + 1,
                end_ind: match self.tokens[self.parent_ind] {
                    TextToken::Array { end, .. } | TextToken::Object { end, .. } => end,
                    _ => self.tokens.len(),
                },
                encoding: self.encoding.clone(),
            }),

            _ => Err(DeserializeError {
                kind: DeserializeErrorKind::Unsupported("not an object"),
            }),
        }
    }

    /// Interpret the current value as an array
    #[inline]
    pub fn read_array(&self) -> Result<ArrayReader<'data, 'tokens, E>, DeserializeError> {
        match self.tokens[self.value_ind] {
            TextToken::Object { end, mixed: true } => {
                let mut start_ind = self.value_ind + 1;
                while self.tokens.get(start_ind) != Some(&TextToken::MixedContainer) {
                    start_ind = next_idx(self.tokens, start_ind);
                }

                Ok(ArrayReader {
                    tokens: self.tokens,
                    start_ind: start_ind + 1,
                    end_ind: end,
                    encoding: self.encoding.clone(),
                })
            }
            TextToken::Array { end, .. } | TextToken::Object { end, .. } => Ok(ArrayReader {
                tokens: self.tokens,
                start_ind: self.value_ind + 1,
                end_ind: end,
                encoding: self.encoding.clone(),
            }),

            // A header can be seen as a two element array
            TextToken::Header(_) => Ok(ArrayReader {
                tokens: self.tokens,
                start_ind: self.value_ind,
                end_ind: next_idx(self.tokens, self.value_ind + 1),
                encoding: self.encoding.clone(),
            }),

            _ => Err(DeserializeError {
                kind: DeserializeErrorKind::Unsupported("not an array"),
            }),
        }
    }

    /// Return the number of tokens the value encompases
    ///
    /// ```
    /// use jomini::TextTape;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let tape = TextTape::from_slice(b"obj={1 {foo=bar} 3}")?;
    /// let reader = tape.windows1252_reader();
    /// let mut fields = reader.fields();
    /// let (_, _, first_value) = fields.next().unwrap();
    /// assert_eq!(first_value.tokens_len(), 6);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn tokens_len(&self) -> usize {
        match self.tokens[self.value_ind] {
            TextToken::Array { end, .. } | TextToken::Object { end, .. } => {
                end - self.value_ind - 1
            }
            _ => 1,
        }
    }
}

/// An iterator over the values of an array
///
/// ```
/// use jomini::TextTape;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let tape = TextTape::from_slice(b"cores={a b}")?;
/// let reader = tape.windows1252_reader();
///
/// let mut all_cores = Vec::new();
/// for (key, _op, value) in reader.fields() {
///     assert_eq!(key.read_str(), "cores");
///     let cores = value.read_array()?;
///     assert_eq!(cores.len(), 2);
///     for value in cores.values() {
///         all_cores.push(value.read_string()?);
///     }
/// }
/// assert_eq!(all_cores, vec![String::from("a"), String::from("b")]);
/// # Ok(())
/// # }
/// ```
pub struct ValuesIter<'data, 'tokens, E> {
    start_ind: usize,
    token_ind: usize,
    end_ind: usize,
    tokens: &'tokens [TextToken<'data>],
    encoding: E,
}

impl<'data, 'tokens, E> ValuesIter<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    fn new(reader: &ArrayReader<'data, 'tokens, E>) -> Self {
        ValuesIter {
            start_ind: reader.start_ind.saturating_sub(1),
            token_ind: reader.start_ind,
            end_ind: reader.end_ind,
            tokens: reader.tokens,
            encoding: reader.encoding.clone(),
        }
    }
}

impl<'data, 'tokens, E> Iterator for ValuesIter<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    type Item = ValueReader<'data, 'tokens, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.token_ind < self.end_ind {
            let value_ind = self.token_ind;
            self.token_ind = next_idx_values(self.tokens, self.token_ind);
            Some(ValueReader {
                parent_ind: self.start_ind,
                value_ind,
                tokens: self.tokens,
                encoding: self.encoding.clone(),
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = values_len(self.tokens, self.token_ind, self.end_ind);
        (len, Some(len))
    }
}

/// A text reader for sequences of values
#[derive(Debug, Clone)]
pub struct ArrayReader<'data, 'tokens, E> {
    start_ind: usize,
    end_ind: usize,
    tokens: &'tokens [TextToken<'data>],
    encoding: E,
}

impl<'data, 'tokens, E> ArrayReader<'data, 'tokens, E>
where
    E: Encoding + Clone,
{
    /// Iterator over values of an array
    ///
    /// See [ValuesIter](crate::jomini::text::ValuesIter) for a worked example
    #[inline]
    pub fn values(&self) -> ValuesIter<'data, 'tokens, E> {
        ValuesIter::new(self)
    }

    /// Returns if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the number of values in the array
    #[inline]
    pub fn len(&self) -> usize {
        values_len(self.tokens, self.start_ind, self.end_ind)
    }

    /// Return the number of tokens contained within the object
    ///
    /// ```
    /// use jomini::TextTape;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let tape = TextTape::from_slice(b"obj={1 {foo=bar} 3}")?;
    /// let reader = tape.windows1252_reader();
    /// let mut fields = reader.fields();
    /// let (_, _, first_value) = fields.next().unwrap();
    /// let array = first_value.read_array()?;
    /// assert_eq!(array.tokens_len(), 6);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn tokens_len(&self) -> usize {
        self.end_ind - self.start_ind
    }
}

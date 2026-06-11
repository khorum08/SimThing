//! Tape → lossless raw model (CT-0b).

use crate::error::ParseError;
use crate::jomini::text::{ArrayReader, ObjectReader, Operator, ScalarReader, ValueReader};
use crate::jomini::{TextTape, TextToken, Windows1252Encoding};
use crate::raw::{
    RawArray, RawBlock, RawDocument, RawHeaderValue, RawOperator, RawProperty, RawScalar, RawSpan,
    RawValue, ScalarForm,
};

type W1252Reader<'data, 'tokens> = ObjectReader<'data, 'tokens, Windows1252Encoding>;
type W1252Value<'data, 'tokens> = ValueReader<'data, 'tokens, Windows1252Encoding>;
type W1252Array<'data, 'tokens> = ArrayReader<'data, 'tokens, Windows1252Encoding>;
type W1252Scalar<'data> = ScalarReader<'data, Windows1252Encoding>;

/// Parse ClauseScript-shaped text into the CT-0b raw document model.
pub fn parse_raw_document(source: &[u8]) -> Result<RawDocument, ParseError> {
    let tape = TextTape::from_slice(source)?;
    let reader = tape.windows1252_reader();
    let root = RawValue::Block(parse_block(reader, false)?);
    Ok(RawDocument { root })
}

fn parse_block(reader: W1252Reader<'_, '_>, mixed: bool) -> Result<RawBlock, ParseError> {
    let mut properties = Vec::new();
    let mut fields = reader.fields();
    while let Some((key, op, value)) = fields.next() {
        let key_index = key_token_index(value.value_index(), op);
        properties.push(RawProperty {
            key: raw_scalar_from_reader(key, key_index),
            operator: op.map(raw_operator_from),
            value: parse_value(value)?,
        });
    }

    let remainder = fields.remainder();
    let tail = if remainder.len() == 0 {
        None
    } else {
        Some(parse_array(remainder, mixed)?)
    };

    Ok(RawBlock {
        properties,
        mixed,
        tail,
    })
}

fn parse_array(reader: W1252Array<'_, '_>, mixed: bool) -> Result<RawArray, ParseError> {
    let items = reader
        .values()
        .map(parse_value)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RawArray { items, mixed })
}

fn parse_value(reader: W1252Value<'_, '_>) -> Result<RawValue, ParseError> {
    let token_index = reader.value_index();
    let token = reader.token().clone();
    match token {
        TextToken::Object { mixed, .. } => {
            Ok(RawValue::Block(parse_block(reader.read_object()?, mixed)?))
        }
        TextToken::MixedContainer => Ok(RawValue::Block(parse_block(reader.read_object()?, true)?)),
        TextToken::Array { mixed, .. } => {
            Ok(RawValue::Array(parse_array(reader.read_array()?, mixed)?))
        }
        TextToken::Header(_) => {
            let array = reader.read_array()?;
            let mut values = array.values();
            let header_reader = values
                .next()
                .ok_or_else(|| ParseError::new("header value missing payload"))?;
            let header_index = header_reader.value_index();
            let payload_reader = values
                .next()
                .ok_or_else(|| ParseError::new("header value missing payload"))?;
            Ok(RawValue::Header(RawHeaderValue {
                header: raw_scalar_from_value(header_reader, header_index)?,
                payload: Box::new(parse_value(payload_reader)?),
            }))
        }
        TextToken::Unquoted(_)
        | TextToken::Quoted(_)
        | TextToken::Parameter(_)
        | TextToken::UndefinedParameter(_) => Ok(RawValue::Scalar(raw_scalar_from_value(
            reader,
            token_index,
        )?)),
        TextToken::Operator(_) => Err(ParseError::new(
            "bare operator token is not a supported raw value",
        )),
        other => Err(ParseError::new(format!(
            "unsupported value token for CT-0b raw model: {other:?}"
        ))),
    }
}

fn raw_scalar_from_reader(reader: W1252Scalar<'_>, token_index: usize) -> RawScalar {
    RawScalar {
        form: scalar_form_from_token(reader.token()),
        text: reader.read_string(),
        span: RawSpan { token_index },
    }
}

fn raw_scalar_from_value(
    reader: W1252Value<'_, '_>,
    token_index: usize,
) -> Result<RawScalar, ParseError> {
    Ok(RawScalar {
        form: scalar_form_from_token(reader.token()),
        text: reader.read_string()?,
        span: RawSpan { token_index },
    })
}

fn scalar_form_from_token(token: &TextToken<'_>) -> ScalarForm {
    match token {
        TextToken::Quoted(_) => ScalarForm::Quoted,
        TextToken::Unquoted(_) => ScalarForm::Unquoted,
        TextToken::Parameter(_) => ScalarForm::Parameter,
        TextToken::UndefinedParameter(_) => ScalarForm::UndefinedParameter,
        TextToken::Header(_) => ScalarForm::Header,
        _ => ScalarForm::Unquoted,
    }
}

fn key_token_index(value_index: usize, operator: Option<Operator>) -> usize {
    if operator.is_some() {
        value_index.saturating_sub(2)
    } else {
        value_index.saturating_sub(1)
    }
}

fn raw_operator_from(operator: Operator) -> RawOperator {
    match operator {
        Operator::Equal => RawOperator::Equal,
        Operator::LessThan => RawOperator::LessThan,
        Operator::LessThanEqual => RawOperator::LessThanEqual,
        Operator::GreaterThan => RawOperator::GreaterThan,
        Operator::GreaterThanEqual => RawOperator::GreaterThanEqual,
        Operator::NotEqual => RawOperator::NotEqual,
        Operator::Exact => RawOperator::Exact,
        Operator::Exists => RawOperator::Exists,
    }
}

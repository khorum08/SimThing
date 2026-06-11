//! Raw model → ClauseScript-shaped text (CT-0b round-trip layer).

use crate::error::EmitError;
use crate::jomini::TextWriterBuilder;
use crate::jomini::text::Operator;
use crate::raw::{
    RawArray, RawBlock, RawDocument, RawOperator, RawProperty, RawScalar, RawValue, ScalarForm,
};

/// Re-emit a raw document as ClauseScript-shaped text bytes.
///
/// Whitespace and comments are normalized; structural semantics preserved for CT-0b.
pub fn emit_text(document: &RawDocument) -> Result<Vec<u8>, EmitError> {
    let mut out = Vec::new();
    let mut writer = TextWriterBuilder::new().from_writer(&mut out);
    match &document.root {
        RawValue::Block(block) => emit_root_block(&mut writer, block)?,
        other => emit_value(&mut writer, other)?,
    }
    Ok(out)
}

fn emit_root_block<W: std::io::Write>(
    writer: &mut crate::jomini::TextWriter<W>,
    block: &RawBlock,
) -> Result<(), EmitError> {
    for property in &block.properties {
        emit_property(writer, property)?;
    }
    if let Some(tail) = &block.tail {
        if block.mixed {
            writer.begin_mixed_tail()?;
        }
        for item in &tail.items {
            emit_value(writer, item)?;
        }
    }
    Ok(())
}

fn emit_value<W: std::io::Write>(
    writer: &mut crate::jomini::TextWriter<W>,
    value: &RawValue,
) -> Result<(), EmitError> {
    match value {
        RawValue::Scalar(scalar) => emit_scalar(writer, scalar),
        RawValue::Block(block) => emit_block(writer, block),
        RawValue::Array(array) => emit_array(writer, array),
        RawValue::Header(header) => {
            emit_scalar(writer, &header.header)?;
            emit_value(writer, &header.payload)
        }
    }
}

fn emit_block<W: std::io::Write>(
    writer: &mut crate::jomini::TextWriter<W>,
    block: &RawBlock,
) -> Result<(), EmitError> {
    writer.write_object_start()?;
    for property in &block.properties {
        emit_property(writer, property)?;
    }
    if let Some(tail) = &block.tail {
        if block.mixed {
            writer.begin_mixed_tail()?;
        }
        for item in &tail.items {
            emit_value(writer, item)?;
        }
    }
    writer.write_end()?;
    Ok(())
}

fn emit_array<W: std::io::Write>(
    writer: &mut crate::jomini::TextWriter<W>,
    array: &RawArray,
) -> Result<(), EmitError> {
    writer.write_array_start()?;
    for item in &array.items {
        emit_value(writer, item)?;
    }
    writer.write_end()?;
    Ok(())
}

fn emit_property<W: std::io::Write>(
    writer: &mut crate::jomini::TextWriter<W>,
    property: &RawProperty,
) -> Result<(), EmitError> {
    emit_scalar(writer, &property.key)?;
    if let Some(operator) = &property.operator {
        let jomini_op = jomini_operator_from(operator);
        if jomini_op != Operator::Equal {
            writer.write_operator(jomini_op)?;
        }
    }
    emit_value(writer, &property.value)
}

fn emit_scalar<W: std::io::Write>(
    writer: &mut crate::jomini::TextWriter<W>,
    scalar: &RawScalar,
) -> Result<(), EmitError> {
    let bytes = scalar.text.as_bytes();
    match scalar.form {
        ScalarForm::Quoted => writer.write_quoted(bytes),
        ScalarForm::Unquoted
        | ScalarForm::Parameter
        | ScalarForm::UndefinedParameter
        | ScalarForm::Header => writer.write_unquoted(bytes),
    }
    .map_err(|err| EmitError::new(err.to_string()))
}

fn jomini_operator_from(operator: &RawOperator) -> Operator {
    match operator {
        RawOperator::Equal => Operator::Equal,
        RawOperator::LessThan => Operator::LessThan,
        RawOperator::LessThanEqual => Operator::LessThanEqual,
        RawOperator::GreaterThan => Operator::GreaterThan,
        RawOperator::GreaterThanEqual => Operator::GreaterThanEqual,
        RawOperator::NotEqual => Operator::NotEqual,
        RawOperator::Exact => Operator::Exact,
        RawOperator::Exists => Operator::Exists,
    }
}

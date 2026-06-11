//! CT-0c expansion passes over the CT-0b raw model.
//!
//! Binding pass order (production track §5 CT-0c; spec §3):
//! 1. scripted variables (`@name`) — document-local definitions plus synthetic map;
//! 2. `inline_script` inclusion with `$PARAM$` substitution and `[[PARAM]]` /
//!    `[[!PARAM]]` conditional inclusion (re-run inside included content);
//! 3. `@[ ... ]` inline math recognized and preserved symbolically — never evaluated;
//! 4. `value:` references left symbolic.
//!
//! Everything here is structural, compile-time expansion over [`RawDocument`].
//! No scope resolution, no hydration to `simthing-spec`, no runtime semantics.
//!
//! Deterministic rules adopted by this rung (documented for CT-0c report):
//! - Only **top-level** `@name = <scalar>` properties are scripted-variable
//!   definitions; they are stripped from the expanded output. Document-local
//!   definitions override synthetic [`ExpansionInput::variables`] entries.
//!   Nested `@name = ...` properties are not definitions and are left symbolic.
//! - `@name` references substitute only when the **entire** unquoted scalar text
//!   equals a defined variable name. Unknown `@name` references stay symbolic.
//! - `$NAME$` substitution applies to **unquoted** scalars (keys and values);
//!   quoted scalars are never substituted. An unresolved `$NAME$` segment under
//!   an active parameter environment is a spanned diagnostic.
//! - A `[[PARAM]]` body is included iff the parameter is defined and its value
//!   is not `"no"`; `[[!PARAM]]` is the exact inverse. Bodies must be blocks;
//!   included properties splice at the conditional's position in source order.
//! - `inline_script` includes splice the library document's top-level properties
//!   at the call site, in source order. Call-block properties other than
//!   `script` become the include's parameter environment (replacing, not
//!   extending, the caller's environment). Recursion is detected by an include
//!   stack and additionally bounded by [`ExpansionOptions::max_inline_depth`].
//! - `@[ ... ]` scalars receive `$PARAM$` substitution **inside** their text but
//!   are otherwise preserved verbatim as symbolic raw scalars.

use std::collections::BTreeMap;

use crate::error::ExpandError;
use crate::raw::{
    RawArray, RawBlock, RawDocument, RawHeaderValue, RawProperty, RawScalar, RawValue, ScalarForm,
};

/// Synthetic expansion inputs for CT-0c (no filesystem reads; tests supply maps).
#[derive(Debug, Clone, Default)]
pub struct ExpansionInput {
    /// Synthetic scripted variables (`name` keyed **with** the leading `@`).
    pub variables: BTreeMap<String, String>,
    /// Synthetic inline-script library keyed by script path/name.
    pub inline_scripts: BTreeMap<String, RawDocument>,
    /// Top-level parameter environment for the root document.
    pub parameters: BTreeMap<String, String>,
    /// Expansion limits.
    pub options: ExpansionOptions,
}

/// Expansion limits with explicit, deterministic caps.
#[derive(Debug, Clone)]
pub struct ExpansionOptions {
    /// Maximum `inline_script` nesting depth (root document is depth 0).
    pub max_inline_depth: usize,
}

impl Default for ExpansionOptions {
    fn default() -> Self {
        Self {
            max_inline_depth: 8,
        }
    }
}

/// Returns true when a scalar is an `@[ ... ]` inline-math marker.
///
/// CT-0c recognizes and preserves these symbolically; nothing evaluates them.
pub fn is_inline_math(scalar: &RawScalar) -> bool {
    scalar.form == ScalarForm::Unquoted
        && scalar.text.starts_with("@[")
        && scalar.text.ends_with(']')
}

/// Returns true when a scalar is a symbolic `value:` reference.
pub fn is_value_reference(scalar: &RawScalar) -> bool {
    scalar.form == ScalarForm::Unquoted && scalar.text.starts_with("value:")
}

struct Env<'a> {
    variables: BTreeMap<String, String>,
    inline_scripts: &'a BTreeMap<String, RawDocument>,
    max_inline_depth: usize,
}

struct Frame {
    parameters: BTreeMap<String, String>,
    include_stack: Vec<String>,
}

/// Expand a raw document per the CT-0c pass order.
pub fn expand_document(
    document: &RawDocument,
    input: &ExpansionInput,
) -> Result<RawDocument, ExpandError> {
    let RawValue::Block(root) = &document.root else {
        return Err(ExpandError::new(
            "expansion requires a block-rooted document",
            None,
        ));
    };

    // Pass 1: collect document-local scripted variables (top level only) and
    // strip their definition properties. Document-local wins over synthetic.
    let mut variables = input.variables.clone();
    let mut retained = Vec::with_capacity(root.properties.len());
    for property in &root.properties {
        if is_variable_definition(property) {
            let RawValue::Scalar(value) = &property.value else {
                return Err(ExpandError::new(
                    format!(
                        "scripted variable `{}` must be a scalar definition",
                        property.key.text
                    ),
                    Some(property.key.span.clone()),
                ));
            };
            variables.insert(property.key.text.clone(), value.text.clone());
        } else {
            retained.push(property.clone());
        }
    }

    let env = Env {
        variables,
        inline_scripts: &input.inline_scripts,
        max_inline_depth: input.options.max_inline_depth,
    };
    let frame = Frame {
        parameters: input.parameters.clone(),
        include_stack: Vec::new(),
    };

    let expanded = expand_properties(&retained, root.mixed, root.tail.as_ref(), &env, &frame)?;
    Ok(RawDocument {
        root: RawValue::Block(expanded),
    })
}

fn is_variable_definition(property: &RawProperty) -> bool {
    property.key.form == ScalarForm::Unquoted
        && property.key.text.starts_with('@')
        && !property.key.text.starts_with("@[")
}

fn expand_properties(
    properties: &[RawProperty],
    mixed: bool,
    tail: Option<&RawArray>,
    env: &Env<'_>,
    frame: &Frame,
) -> Result<RawBlock, ExpandError> {
    let mut out = Vec::with_capacity(properties.len());
    for property in properties {
        match classify(property) {
            PropertyKind::ConditionalPresent => {
                splice_conditional(&mut out, property, true, env, frame)?;
            }
            PropertyKind::ConditionalAbsent => {
                splice_conditional(&mut out, property, false, env, frame)?;
            }
            PropertyKind::InlineScript => {
                splice_inline_script(&mut out, property, env, frame)?;
            }
            PropertyKind::Plain => {
                out.push(RawProperty {
                    key: substitute_scalar(&property.key, env, frame)?,
                    operator: property.operator.clone(),
                    value: expand_value(&property.value, env, frame)?,
                });
            }
        }
    }
    let tail = match tail {
        None => None,
        Some(array) => Some(expand_array(array, env, frame)?),
    };
    Ok(RawBlock {
        properties: out,
        mixed,
        tail,
    })
}

enum PropertyKind {
    ConditionalPresent,
    ConditionalAbsent,
    InlineScript,
    Plain,
}

fn classify(property: &RawProperty) -> PropertyKind {
    match property.key.form {
        ScalarForm::Parameter => PropertyKind::ConditionalPresent,
        ScalarForm::UndefinedParameter => PropertyKind::ConditionalAbsent,
        _ if property.key.text == "inline_script" => PropertyKind::InlineScript,
        _ => PropertyKind::Plain,
    }
}

fn parameter_truthy(frame: &Frame, name: &str) -> bool {
    matches!(frame.parameters.get(name), Some(value) if value != "no")
}

fn splice_conditional(
    out: &mut Vec<RawProperty>,
    property: &RawProperty,
    wants_present: bool,
    env: &Env<'_>,
    frame: &Frame,
) -> Result<(), ExpandError> {
    let truthy = parameter_truthy(frame, &property.key.text);
    let included = if wants_present { truthy } else { !truthy };
    if !included {
        return Ok(());
    }
    let RawValue::Block(body) = &property.value else {
        return Err(ExpandError::new(
            format!(
                "conditional parameter `{}` body must be a block",
                property.key.text
            ),
            Some(property.key.span.clone()),
        ));
    };
    if body.tail.is_some() {
        return Err(ExpandError::new(
            format!(
                "conditional parameter `{}` body with a mixed tail is not supported at CT-0c",
                property.key.text
            ),
            Some(property.key.span.clone()),
        ));
    }
    let expanded = expand_properties(&body.properties, body.mixed, None, env, frame)?;
    out.extend(expanded.properties);
    Ok(())
}

fn splice_inline_script(
    out: &mut Vec<RawProperty>,
    property: &RawProperty,
    env: &Env<'_>,
    frame: &Frame,
) -> Result<(), ExpandError> {
    let (name_scalar, parameters) = match &property.value {
        RawValue::Scalar(scalar) => (substitute_scalar(scalar, env, frame)?, BTreeMap::new()),
        RawValue::Block(block) => {
            let mut script = None;
            let mut parameters = BTreeMap::new();
            for entry in &block.properties {
                let RawValue::Scalar(value) = &entry.value else {
                    return Err(ExpandError::new(
                        format!(
                            "inline_script entry `{}` must be a scalar at CT-0c",
                            entry.key.text
                        ),
                        Some(entry.key.span.clone()),
                    ));
                };
                let value = substitute_scalar(value, env, frame)?;
                if entry.key.text == "script" {
                    script = Some(value);
                } else {
                    parameters.insert(entry.key.text.clone(), value.text);
                }
            }
            let Some(script) = script else {
                return Err(ExpandError::new(
                    "inline_script block is missing its `script` entry",
                    Some(property.key.span.clone()),
                ));
            };
            (script, parameters)
        }
        _ => {
            return Err(ExpandError::new(
                "inline_script value must be a scalar path or a block",
                Some(property.key.span.clone()),
            ));
        }
    };

    let name = name_scalar.text;
    if frame.include_stack.iter().any(|entry| entry == &name) {
        return Err(ExpandError::new(
            format!(
                "recursive inline_script inclusion detected: `{name}` (stack: {})",
                frame.include_stack.join(" -> ")
            ),
            Some(property.key.span.clone()),
        ));
    }
    if frame.include_stack.len() + 1 > env.max_inline_depth {
        return Err(ExpandError::new(
            format!(
                "inline_script depth cap {} exceeded at `{name}`",
                env.max_inline_depth
            ),
            Some(property.key.span.clone()),
        ));
    }
    let Some(library_document) = env.inline_scripts.get(&name) else {
        return Err(ExpandError::new(
            format!("inline_script target `{name}` is not in the synthetic library"),
            Some(property.key.span.clone()),
        ));
    };
    let RawValue::Block(body) = &library_document.root else {
        return Err(ExpandError::new(
            format!("inline_script target `{name}` must be a block-rooted document"),
            Some(property.key.span.clone()),
        ));
    };
    if body.tail.is_some() {
        return Err(ExpandError::new(
            format!("inline_script target `{name}` with a mixed tail is not supported at CT-0c"),
            Some(property.key.span.clone()),
        ));
    }

    let mut include_stack = frame.include_stack.clone();
    include_stack.push(name);
    let include_frame = Frame {
        parameters,
        include_stack,
    };
    let expanded = expand_properties(&body.properties, body.mixed, None, env, &include_frame)?;
    out.extend(expanded.properties);
    Ok(())
}

fn expand_value(value: &RawValue, env: &Env<'_>, frame: &Frame) -> Result<RawValue, ExpandError> {
    match value {
        RawValue::Scalar(scalar) => Ok(RawValue::Scalar(substitute_scalar(scalar, env, frame)?)),
        RawValue::Block(block) => Ok(RawValue::Block(expand_properties(
            &block.properties,
            block.mixed,
            block.tail.as_ref(),
            env,
            frame,
        )?)),
        RawValue::Array(array) => Ok(RawValue::Array(expand_array(array, env, frame)?)),
        RawValue::Header(header) => Ok(RawValue::Header(RawHeaderValue {
            header: substitute_scalar(&header.header, env, frame)?,
            payload: Box::new(expand_value(&header.payload, env, frame)?),
        })),
    }
}

fn expand_array(array: &RawArray, env: &Env<'_>, frame: &Frame) -> Result<RawArray, ExpandError> {
    let items = array
        .items
        .iter()
        .map(|item| expand_value(item, env, frame))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RawArray {
        items,
        mixed: array.mixed,
    })
}

fn substitute_scalar(
    scalar: &RawScalar,
    env: &Env<'_>,
    frame: &Frame,
) -> Result<RawScalar, ExpandError> {
    if scalar.form != ScalarForm::Unquoted {
        return Ok(scalar.clone());
    }
    // Whole-token scripted variable reference (never inline math, which also
    // starts with `@`).
    if scalar.text.starts_with('@') && !scalar.text.starts_with("@[") {
        if let Some(value) = env.variables.get(&scalar.text) {
            return Ok(RawScalar {
                form: ScalarForm::Unquoted,
                text: value.clone(),
                span: scalar.span.clone(),
            });
        }
        // Unknown `@name` forms (including dynamic identifiers) stay symbolic.
        return Ok(scalar.clone());
    }
    let text = substitute_parameters(&scalar.text, frame, &scalar.span)?;
    Ok(RawScalar {
        form: ScalarForm::Unquoted,
        text,
        span: scalar.span.clone(),
    })
}

fn substitute_parameters(
    text: &str,
    frame: &Frame,
    span: &crate::raw::RawSpan,
) -> Result<String, ExpandError> {
    if !text.contains('$') {
        return Ok(text.to_string());
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    loop {
        let Some(start) = rest.find('$') else {
            out.push_str(rest);
            return Ok(out);
        };
        let Some(end_rel) = rest[start + 1..].find('$') else {
            // Unpaired `$` is preserved verbatim (documented rule).
            out.push_str(rest);
            return Ok(out);
        };
        let name = &rest[start + 1..start + 1 + end_rel];
        out.push_str(&rest[..start]);
        let Some(value) = frame.parameters.get(name) else {
            return Err(ExpandError::new(
                format!("parameter `${name}$` is not defined in the active environment"),
                Some(span.clone()),
            ));
        };
        out.push_str(value);
        rest = &rest[start + 2 + end_rel..];
    }
}

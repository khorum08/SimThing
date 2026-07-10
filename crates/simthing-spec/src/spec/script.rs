use serde::{Deserialize, Serialize};
use simthing_core::{DimensionRegistry, SimPropertyId, SubFieldRole};

/// Symbolic property reference used by authored scripts.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropertyKey {
    pub namespace: String,
    pub name: String,
}

impl PropertyKey {
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }
}

/// Scope for a script read. PR 7 intentionally keeps scope resolution small:
/// callers either evaluate against the current slot or explicitly name a slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ScopeRef {
    Current,
    Slot(u32),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum ScriptExpr {
    Const(f32),
    Read {
        scope: ScopeRef,
        property: PropertyKey,
        role: SubFieldRole,
    },
    Add(Box<ScriptExpr>, Box<ScriptExpr>),
    Sub(Box<ScriptExpr>, Box<ScriptExpr>),
    Mul(Box<ScriptExpr>, Box<ScriptExpr>),
    Div(Box<ScriptExpr>, Box<ScriptExpr>),
    Min(Box<ScriptExpr>, Box<ScriptExpr>),
    Max(Box<ScriptExpr>, Box<ScriptExpr>),
    Clamp {
        value: Box<ScriptExpr>,
        min: f32,
        max: f32,
    },
    Gate(Box<ScriptPredicate>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[non_exhaustive]
pub enum ScriptPredicate {
    True,
    False,
    Greater(ScriptExpr, ScriptExpr),
    Less(ScriptExpr, ScriptExpr),
    Equalish(ScriptExpr, ScriptExpr),
    And(Vec<ScriptPredicate>),
    Or(Vec<ScriptPredicate>),
    Not(Box<ScriptPredicate>),
}

pub struct ScriptEvalContext<'a> {
    pub registry: &'a DimensionRegistry,
    pub shadow: &'a [f32],
    pub n_dims: usize,
    pub current_slot: u32,
}

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub enum ScriptEvalError {
    #[error("script references unknown property `{namespace}::{name}`")]
    UnknownProperty { namespace: String, name: String },
    #[error("script references role `{role:?}` not present on property `{property_id:?}`")]
    UnknownRole {
        property_id: SimPropertyId,
        role: SubFieldRole,
    },
    #[error("script scope references slot {slot}, but shadow only has {slots} slots")]
    SlotOutOfBounds { slot: u32, slots: usize },
    #[error("script read resolved to column {col}, but n_dims is {n_dims}")]
    ColumnOutOfBounds { col: usize, n_dims: usize },
    #[error("script attempted division by zero")]
    DivisionByZero,
    #[error("script clamp has min {min} greater than max {max}")]
    InvalidClamp { min: f32, max: f32 },
}

impl ScriptExpr {
    pub fn eval(&self, ctx: &ScriptEvalContext<'_>) -> Result<f32, ScriptEvalError> {
        match self {
            Self::Const(value) => Ok(*value),
            Self::Read {
                scope,
                property,
                role,
            } => read_value(ctx, scope, property, role),
            Self::Add(lhs, rhs) => Ok(lhs.eval(ctx)? + rhs.eval(ctx)?),
            Self::Sub(lhs, rhs) => Ok(lhs.eval(ctx)? - rhs.eval(ctx)?),
            Self::Mul(lhs, rhs) => Ok(lhs.eval(ctx)? * rhs.eval(ctx)?),
            Self::Div(lhs, rhs) => {
                let denominator = rhs.eval(ctx)?;
                if denominator == 0.0 {
                    return Err(ScriptEvalError::DivisionByZero);
                }
                Ok(lhs.eval(ctx)? / denominator)
            }
            Self::Min(lhs, rhs) => Ok(lhs.eval(ctx)?.min(rhs.eval(ctx)?)),
            Self::Max(lhs, rhs) => Ok(lhs.eval(ctx)?.max(rhs.eval(ctx)?)),
            Self::Clamp { value, min, max } => {
                if min > max {
                    return Err(ScriptEvalError::InvalidClamp {
                        min: *min,
                        max: *max,
                    });
                }
                Ok(value.eval(ctx)?.clamp(*min, *max))
            }
            Self::Gate(predicate) => Ok(if predicate.eval(ctx)? { 1.0 } else { 0.0 }),
        }
    }
}

impl ScriptPredicate {
    pub fn eval(&self, ctx: &ScriptEvalContext<'_>) -> Result<bool, ScriptEvalError> {
        match self {
            Self::True => Ok(true),
            Self::False => Ok(false),
            Self::Greater(lhs, rhs) => Ok(lhs.eval(ctx)? > rhs.eval(ctx)?),
            Self::Less(lhs, rhs) => Ok(lhs.eval(ctx)? < rhs.eval(ctx)?),
            Self::Equalish(lhs, rhs) => Ok((lhs.eval(ctx)? - rhs.eval(ctx)?).abs() <= 0.000_1),
            Self::And(predicates) => {
                for predicate in predicates {
                    if !predicate.eval(ctx)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Self::Or(predicates) => {
                for predicate in predicates {
                    if predicate.eval(ctx)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            Self::Not(predicate) => Ok(!predicate.eval(ctx)?),
        }
    }
}

fn read_value(
    ctx: &ScriptEvalContext<'_>,
    scope: &ScopeRef,
    property: &PropertyKey,
    role: &SubFieldRole,
) -> Result<f32, ScriptEvalError> {
    let property_id = ctx
        .registry
        .id_of(&property.namespace, &property.name)
        .ok_or_else(|| ScriptEvalError::UnknownProperty {
            namespace: property.namespace.clone(),
            name: property.name.clone(),
        })?;
    let layout = &ctx.registry.property(property_id).layout;
    let range = ctx.registry.column_range(property_id);
    let col = range
        .col_for_role(role, layout)
        .ok_or_else(|| ScriptEvalError::UnknownRole {
            property_id,
            role: role.clone(),
        })?;
    if col.raw() >= ctx.n_dims {
        return Err(ScriptEvalError::ColumnOutOfBounds {
            col: col.raw(),
            n_dims: ctx.n_dims,
        });
    }

    let slot = match scope {
        ScopeRef::Current => ctx.current_slot,
        ScopeRef::Slot(slot) => *slot,
    };
    let slots = if ctx.n_dims == 0 {
        0
    } else {
        ctx.shadow.len() / ctx.n_dims
    };
    if slot as usize >= slots {
        return Err(ScriptEvalError::SlotOutOfBounds { slot, slots });
    }

    Ok(ctx.shadow[slot as usize * ctx.n_dims + col.raw()])
}

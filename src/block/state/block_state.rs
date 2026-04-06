use crate::utils::utils;
use std::collections::HashSet;
use std::hash::Hash;

pub enum BlockStateDefinition {
    Bool {
        identifier: &'static str,
        default: bool,
    },
    Int {
        identifier: &'static str,
        min: i32,
        max: i32,
    },
    Enum {
        identifier: &'static str,
        values: &'static [&'static str],
    },
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum BlockState {
    Bool(bool),
    Int(i32),
    Enum(&'static str),
}

impl BlockStateDefinition {
    pub const fn new_int(identifier: &'static str, min: i32, max: i32) -> BlockStateDefinition {
        BlockStateDefinition::Int {
            identifier,
            min,
            max,
        }
    }

    pub const fn new_bool(identifier: &'static str, default: bool) -> BlockStateDefinition {
        BlockStateDefinition::Bool {
            identifier,
            default,
        }
    }

    pub const fn new_enum(
        identifier: &'static str,
        values: &'static [&'static str],
    ) -> BlockStateDefinition {
        BlockStateDefinition::Enum { identifier, values }
    }

    pub const fn bit_size(&self) -> u8 {
        match self {
            BlockStateDefinition::Bool { .. } => 1,
            BlockStateDefinition::Int { min, max, .. } => utils::compute_required_bits(*min, *max),
            BlockStateDefinition::Enum { values, .. } => {
                utils::compute_required_bits(0, (values.len() - 1) as i32)
            }
        }
    }

    pub const fn identifier(&self) -> &'static str {
        match self {
            BlockStateDefinition::Int { identifier, .. } => identifier,
            BlockStateDefinition::Bool { identifier, .. } => identifier,
            BlockStateDefinition::Enum { identifier, .. } => identifier,
        }
    }

    pub const fn index_of(&self, state: &BlockState) -> i32 {
        match (self, state) {
            (BlockStateDefinition::Int { min, .. }, BlockState::Int(val)) => *val - *min,
            (BlockStateDefinition::Bool { .. }, BlockState::Bool(val)) => {
                if *val {
                    1
                } else {
                    0
                }
            }
            (BlockStateDefinition::Enum { values, .. }, BlockState::Enum(val)) => {
                let mut i = 0;
                while i < values.len() {
                    if const_str::equal!(values[i], *val) {
                        return i as i32;
                    }
                    i += 1;
                }
                -1
            }
            _ => -1,
        }
    }

    pub fn get_int_values(&self) -> Option<Vec<i32>> {
        match self {
            BlockStateDefinition::Int { min, max, .. } => Some((*min..=*max).collect()),
            _ => None,
        }
    }

    pub fn get_bool_values(&self) -> Option<Vec<bool>> {
        match self {
            BlockStateDefinition::Bool { default, .. } => Some(vec![*default, !*default]),
            _ => None,
        }
    }

    pub fn get_enum_values(&self) -> Option<Vec<&'static str>> {
        match self {
            BlockStateDefinition::Enum { values, .. } => Some(values.to_vec()),
            _ => None,
        }
    }

    pub fn validate<T: Hash + PartialEq + Eq>(
        identifier: &str,
        valid_values: &[T],
    ) -> Result<(), String> {
        let mut set = HashSet::<&T>::with_capacity(valid_values.len());
        if !valid_values.iter().all(|v| set.insert(v)) {
            return Err(format!(
                "BlockState {} must have no duplicate values",
                identifier
            ));
        }

        if valid_values.len() < 2 {
            return Err(format!(
                "BlockState {} must have at least 2 values",
                identifier
            ));
        }

        Ok(())
    }
}

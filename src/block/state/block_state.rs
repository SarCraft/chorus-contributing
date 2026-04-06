use std::collections::HashSet;
use std::hash::Hash;
use crate::utils::identifier::Identifier;

#[derive(Debug)]
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

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum BlockState {
    Bool(bool),
    Int(i32),
    Enum(&'static str),
}

impl BlockStateDefinition {
    pub const fn new_int(identifier: &'static str, min: i32, max: i32) -> BlockStateDefinition {
        let (min, max) = if min < max { (min, max) } else { (max, min) };
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
            (BlockStateDefinition::Bool { default, .. }, BlockState::Bool(val)) => {
                if *val == *default { 0 } else { 1 }
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

    pub const fn values_len(&self) -> usize {
        match self {
            BlockStateDefinition::Bool { .. } => 2,
            BlockStateDefinition::Int { min, max, .. } => (*max - *min + 1) as usize,
            BlockStateDefinition::Enum { values, .. } => values.len(),
        }
    }

    pub fn get_values(&self) -> Vec<BlockState> {
        match self {
            BlockStateDefinition::Bool { default, .. } => {
                vec![BlockState::Bool(*default), BlockState::Bool(!*default)]
            }
            BlockStateDefinition::Int { min, max, .. } => {
                (*min..=*max).map(BlockState::Int).collect()
            }
            BlockStateDefinition::Enum { values, .. } => {
                values.iter().map(|v| BlockState::Enum(*v)).collect()
            }
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

    pub fn validate(&self) -> Result<(), String> {
        Identifier::validate(self.identifier())?;
        
        let valid_values = self.get_values();

        let mut set = HashSet::<BlockState>::with_capacity(valid_values.len());
        if !valid_values.into_iter().all(|v| set.insert(v)) {
            return Err(format!(
                "{:?} must have no duplicate values",
                self.identifier()
            ));
        }

        if set.len() < 2 {
            return Err(format!(
                "{:?} must have at least 2 values",
                self.identifier()
            ));
        }

        Ok(())
    }
}

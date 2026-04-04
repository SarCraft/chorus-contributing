use std::collections::HashSet;
use std::hash::Hash;

pub struct BlockState<'a, T> {
    identifier: &'a str,
    values: Vec<T>,
}

impl<T: PartialEq + Eq + Hash> BlockState<'_, T> {
    pub fn validate(&self) -> Result<(), String> {
        let mut set = HashSet::<&T>::with_capacity(self.values.len());
        if !self.values.iter().all(|v| set.insert(v)) {
            return Err(format!(
                "BlockState {} must have no duplicate values",
                self.identifier
            ));
        }

        if self.values.len() < 2 {
            return Err(format!(
                "BlockState {} must have at least 2 values",
                self.identifier
            ));
        }

        Ok(())
    }
}

pub struct IntBlockState {
    identifier: &'static str,
    min: i32,
    max: i32,
}

impl IntBlockState {
    pub const fn from(identifier: &'static str, min: i32, max: i32) -> Self {
        Self {
            identifier,
            min,
            max,
        }
    }

    pub const fn from_max(identifier: &'static str, max: i32) -> Self {
        Self::from(identifier, 0, max)
    }
}

impl From<IntBlockState> for BlockState<'static, i32> {
    fn from(value: IntBlockState) -> Self {
        Self {
            identifier: value.identifier,
            values: (value.min..=value.max).collect(),
        }
    }
}

pub struct BoolBlockState {
    identifier: &'static str,
    default: bool,
}

impl BoolBlockState {
    pub const fn from_default(identifier: &'static str, default: bool) -> Self {
        Self {
            identifier,
            default,
        }
    }

    pub const fn from(identifier: &'static str) -> Self {
        Self::from_default(identifier, false)
    }
}

impl From<BoolBlockState> for BlockState<'static, bool> {
    fn from(value: BoolBlockState) -> Self {
        Self {
            identifier: value.identifier,
            values: vec![value.default, !value.default],
        }
    }
}

pub struct EnumBlockState {
    identifier: &'static str,
    values: &'static [&'static str],
}

impl EnumBlockState {
    pub const fn from(identifier: &'static str, values: &'static [&'static str]) -> Self {
        Self { identifier, values }
    }
}

impl From<EnumBlockState> for BlockState<'static, &'static str> {
    fn from(value: EnumBlockState) -> Self {
        Self {
            identifier: value.identifier,
            values: value.values.to_vec(),
        }
    }
}

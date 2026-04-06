use crate::block::block_permutation::BlockPermutation;
use crate::block::component::block_component::BlockComponent;
use crate::block::component::block_components::BlockComponents;
use crate::block::state::block_state::{BlockState, BlockStateDefinition};
use crate::utils::identifier::Identifier;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct BlockDefinition {
    pub identifier: &'static str,
    pub states: &'static [&'static BlockStateDefinition],
    pub components: &'static [&'static dyn BlockComponent],
    pub permutations: &'static [&'static BlockPermutationDefinition],
}

#[derive(Debug)]
pub struct BlockPermutationDefinition {
    pub condition: fn(&HashMap<&'static str, BlockState>) -> bool,
    pub components: &'static [&'static dyn BlockComponent],
}

impl BlockDefinition {
    pub fn validate(&self) -> Result<(), String> {
        Identifier::validate(self.identifier)?;

        let size: usize = self.states.iter().fold(1, |acc, s| acc * s.values_len());
        if size > u16::MAX as usize {
            return Err(format!(
                "BlockDefinition {:?} exceeds the permutation limit, found {size} permutations",
                self.identifier,
            ));
        }

        let mut seen = HashSet::new();
        for state in self.states {
            let ident = state.identifier();
            if !seen.insert(ident) {
                return Err(format!(
                    "BlockDefinition {:?} has duplicate state identifier {:?}",
                    self.identifier, ident
                ));
            }
            state.validate()?;
        }
        Ok(())
    }

    pub fn generate(
        &self,
    ) -> (
        i32,
        HashMap<i32, BlockPermutation>,
        HashMap<i32, BlockComponents>,
    ) {
        let state_combinations = self.states.iter().fold(vec![HashMap::new()], |acc, state| {
            acc.into_iter()
                .flat_map(|map| {
                    state.get_values().into_iter().map(move |val| {
                        let mut m = map.clone();
                        m.insert(state.identifier(), val);
                        m
                    })
                })
                .collect()
        });

        let permutations: Vec<BlockPermutation> = state_combinations
            .into_iter()
            .map(move |states| BlockPermutation::new(self, states))
            .collect();

        let default = permutations
            .iter()
            .find(|p| p.get_index() == 0)
            .unwrap()
            .get_hash();

        let components: HashMap<i32, BlockComponents> = permutations
            .iter()
            .map(|permutation| {
                let hash = permutation.get_hash();
                let mut components = BlockComponents::new();

                for c in self.components {
                    components.insert(*c);
                }

                for p in self.permutations {
                    if (p.condition)(permutation.get_states()) {
                        for c in p.components {
                            components.insert(*c);
                        }
                    }
                }

                (hash, components)
            })
            .collect();

        let permutations: HashMap<i32, BlockPermutation> = permutations
            .into_iter()
            .map(|p| (p.get_hash(), p))
            .collect();

        (default, permutations, components)
    }
}

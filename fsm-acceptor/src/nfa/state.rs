use std::collections::HashSet;

use multimap::MultiMap;

use crate::alphabet::Alphabet;

pub type StateId = usize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct State<A: Alphabet> {
    pub id: StateId,
    pub accepting: bool,
    pub(crate) transitions: MultiMap<A, StateId>,
    pub(crate) epsilon_transitions: HashSet<StateId>,
}

impl<A: Alphabet> State<A> {
    pub fn new(id: StateId, accepting: bool) -> Self {
        Self {
            id,
            accepting,
            transitions: MultiMap::new(),
            epsilon_transitions: HashSet::new(),
        }
    }

    pub fn add_transition(&mut self, symbol: A, to: StateId) {
        self.transitions.insert(symbol, to);
    }

    pub fn add_epsilon_transition(&mut self, to: StateId) {
        self.epsilon_transitions.insert(to);
    }

    pub fn next(&self, symbol: A) -> Option<&Vec<StateId>> {
        self.transitions.get_vec(&symbol)
    }

    pub fn next_epsilon(&self) -> &HashSet<StateId> {
        &self.epsilon_transitions
    }
}

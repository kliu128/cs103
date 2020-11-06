#[derive(Debug)]
pub struct DFA<const NUM_STATES: usize, const NUM_INPUTS: usize> {
  pub accepting_states: [bool; NUM_STATES],
  pub states: [[usize; NUM_INPUTS]; NUM_STATES]
}

pub const fn run_dfa<const NUM_STATES: usize, const NUM_INPUTS: usize, const INPUT_LEN: usize>(
  dfa: &DFA<NUM_STATES, NUM_INPUTS>, input: [usize; INPUT_LEN]) -> bool {
    let mut state: usize = 0;
    let mut i = 0;
    while i < input.len() {
      state = dfa.states[state][input[i]];
      i += 1;
    }
    dfa.accepting_states[state]
}
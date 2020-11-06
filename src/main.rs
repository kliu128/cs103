#![feature(min_const_generics)]

mod dfa;

fn main() {
    println!("Welcome to the CS103 DFA solver.");
    let answer = dfa::run_dfa(&dfa::DFA {accepting_states: [true], states: [
        [0]
    ]}, [0, 0, 0]);

    println!("DFA answer: {}", answer);
}

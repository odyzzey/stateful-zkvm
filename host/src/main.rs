use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{ default_executor_from_elf,
    serde::to_vec,
    ExecutorEnv, SessionReceipt,
};
use stateful_core::GET_STATE;
use std::cell::RefCell;

const STATE_LEN: usize = 10;

fn main() {
    let state = RefCell::new(random_state());
    let mut receipts: Vec<SessionReceipt> = Vec::new();

    println!("Initial    {:?}", state.borrow());

    while !is_sorted(&state.borrow()) {
        let env = ExecutorEnv::builder()
            .io_callback(GET_STATE, |input: &[u8]| {
                let addr: u8 = input[0];
                let s = state.borrow();
                [s[addr as usize]].to_vec()
            })
            .add_input(&to_vec(&STATE_LEN).unwrap())
            .build()
            .unwrap();
        let mut exec = default_executor_from_elf(env, METHOD_NAME_ELF).unwrap();
        let session = exec.run().unwrap();
        let receipt = session.prove().unwrap();
        receipt.verify(METHOD_NAME_ID).unwrap();

        let mut s = state.borrow_mut();
        *s = state_transition(&s, &receipt.journal);

        println!("State {: <4} {:?}", receipts.len(), s);

        receipts.push(receipt);
    }
}

fn state_transition(state: &[u8; STATE_LEN], journal: &Vec<u8>) -> [u8; STATE_LEN] {
    let mut state = state.to_owned();
    for word in journal.chunks_exact(8) {
        let (addr, value) = (word[0], word[4]);
        state[addr as usize] = value;
    }
    state
}

fn random_state() -> [u8; STATE_LEN] {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    
    let mut state= [0; STATE_LEN];
    for i in 0..STATE_LEN{
        state[i] = i as u8;
    }

    state.shuffle(&mut rng);
    state
}

fn is_sorted(state: &[u8; STATE_LEN]) -> bool {
    for i in 1..STATE_LEN {
        if state[i-1] > state[i] {
            return false;
        }
    }
    true
}

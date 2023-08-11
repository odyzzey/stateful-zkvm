#![no_main]

use risc0_zkvm::guest::env;
use stateful_core::GET_STATE;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let len: u8 = env::read();

    let mut _left = 0;
    let mut _right = 0;
    let mut index = 0;

    while index < len - 1 {
        _left = get_state(index);
        _right = get_state(index + 1);
        if _left > _right {
            let mut search_index = index + 1;
            while search_index < len {
                let candidate = get_state(search_index);

                if candidate < _left {
                    set_state(index, candidate);
                    set_state(search_index, _left);
                    index = search_index;
                    break;
                }
                search_index += 1;
            }
        }

        index += 1;
    }
}

fn get_state(addr: u8) -> u8 {
    let res = env::send_recv_slice::<u8, u8>(GET_STATE, &[addr]);
    res[0]
}

fn set_state(addr: u8, new_value: u8) {
    env::commit(&(addr, new_value));
}

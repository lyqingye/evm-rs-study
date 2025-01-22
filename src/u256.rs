use alloy_primitives::{Address, FixedBytes, U256};

pub fn u256_to_usize(value: U256) -> usize {
    let limbs = value.as_limbs();
    if limbs[1] == 0 && limbs[2] == 0 && limbs[3] == 0 {
        limbs[0] as usize
    } else {
        usize::MAX
    }
}

pub fn u256_to_address(value: U256) -> Address {
    Address::from_word(FixedBytes(value.to_be_bytes()))
}

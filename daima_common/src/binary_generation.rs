use std::usize;

use crate::constants::{CONTINUEATION_BYTE_MASK, ONE_BYTE_BIT_MASK, SHIFT_AMOUNT_TYPETAG};
const SEVEN_BIT_MASK: usize = 0b0111_1111;

pub fn size_to_bytes_sequence(mut size: usize) -> Vec<u8> {
    let mut to_return = Vec::new();
    loop {
        let mut next_byte: usize = size & SEVEN_BIT_MASK;
        size >>= 7;
        let is_last_byte = size == 0;
        next_byte = if is_last_byte {
            next_byte
        } else {
            next_byte | CONTINUEATION_BYTE_MASK
        };
        to_return.push(next_byte as u8);
        if size == 0 {
            return to_return;
        }
    }
}

pub fn u16_to_bytes(mut value: u16) -> [u8; 2] {
    const ONE_BYTE_BIT_MASK_U16: u16 = ONE_BYTE_BIT_MASK as u16;
    let right = (value & ONE_BYTE_BIT_MASK_U16) as u8;
    value >>= SHIFT_AMOUNT_TYPETAG;
    let left = (value & ONE_BYTE_BIT_MASK_U16) as u8;
    [left, right]
}

#[cfg(test)]
mod testing {
    use crate::binary_generation::size_to_bytes_sequence;

    #[test]
    fn produce_size_bytes_from_size() {
        fn assert_case(given: usize, expected: Vec<u8>) {
            let actual = size_to_bytes_sequence(given);
            assert_eq!(expected, actual, "Given: {}", given);
        }

        assert_case(0, vec![0b0000_0000]);
        assert_case(2, vec![0b0000_0010]);
        assert_case(127, vec![0b0111_1111]);
        assert_case(128, vec![0b1000_0000, 0b0000_0001]);
        assert_case(129, vec![0b1000_0001, 0b0000_0001]);
        assert_case(131, vec![0b1000_0011, 0b0000_0001]);
        assert_case(131, vec![0b1000_0011, 0b0000_0001]);
        assert_case(387, vec![0b1000_0011, 0b0000_0011]);
    }
}

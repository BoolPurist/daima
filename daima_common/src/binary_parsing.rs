// TODO: remove allow dead_code when used
#![allow(dead_code)]

use std::usize;

type NumberBytesSize = u64;
type TypeTagSize = u16;
const SHIF_AMOUNT: NumberBytesSize = 7;
const NEXT_NUMBER_OF_BYTES_MASK: NumberBytesSize = 0b0111_1111;
const CONTINUEATION_BYTE_MASK: NumberBytesSize = 0b1000_0000;

#[derive(Debug, PartialEq, Eq)]
struct ParsedBinaryStream {
    number_of_bytes_payload: NumberBytesSize,
    type_tag: TypeTagSize,
    payload_as_bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
enum ByteStreamReadingState<'s> {
    Length {
        shift_counter: NumberBytesSize,
        current_number_of_bytes: NumberBytesSize,
    },
    TypeTag {
        number_of_bytes_payload: NumberBytesSize,
        number_of_read_tap_type_bytes: u16,
        wip_type_tag: TypeTagSize,
    },
    PayLoad(ParsedBinaryStream),
    Done {
        parsed: ParsedBinaryStream,
        rest: &'s [u8],
    },
}

impl<'s> ByteStreamReadingState<'s> {
    fn start(stream: &'s [u8]) -> Self {
        let not_started = ByteStreamReadingState::Length {
            shift_counter: 0,
            current_number_of_bytes: 0,
        };
        not_started.advance(stream)
    }

    fn advance(self, stream: &'s [u8]) -> Self {
        let mut current_stream = stream;
        match self {
            ByteStreamReadingState::Length {
                mut shift_counter,
                mut current_number_of_bytes,
            } => {
                while !current_stream.is_empty() {
                    let next_byte = current_stream[0] as NumberBytesSize;
                    current_stream = &current_stream[1..];
                    let (is_last_byte, to_append) = (
                        (next_byte & CONTINUEATION_BYTE_MASK) != CONTINUEATION_BYTE_MASK,
                        next_byte & NEXT_NUMBER_OF_BYTES_MASK,
                    );
                    current_number_of_bytes |= to_append << (shift_counter * SHIF_AMOUNT);
                    if is_last_byte {
                        return ByteStreamReadingState::TypeTag {
                            number_of_bytes_payload: current_number_of_bytes,
                            number_of_read_tap_type_bytes: 0,
                            wip_type_tag: 0,
                        }
                        .advance(current_stream);
                    }
                    shift_counter += 1;
                }
                ByteStreamReadingState::Length {
                    shift_counter,
                    current_number_of_bytes,
                }
            }
            ByteStreamReadingState::TypeTag {
                number_of_bytes_payload,
                mut wip_type_tag,
                mut number_of_read_tap_type_bytes,
            } => {
                while !current_stream.is_empty() {
                    let next_byte = current_stream[0];
                    current_stream = &current_stream[1..];
                    let bitmask = next_byte as u16;
                    wip_type_tag |= bitmask << (number_of_read_tap_type_bytes * 8u16);
                    number_of_read_tap_type_bytes += 1;
                    if number_of_read_tap_type_bytes == 2 {
                        return ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                            number_of_bytes_payload,
                            type_tag: wip_type_tag,
                            payload_as_bytes: Vec::with_capacity(number_of_bytes_payload as usize),
                        })
                        .advance(current_stream);
                    }
                }
                ByteStreamReadingState::TypeTag {
                    number_of_bytes_payload,
                    wip_type_tag,
                    number_of_read_tap_type_bytes,
                }
            }
            ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                number_of_bytes_payload,
                type_tag,
                mut payload_as_bytes,
            }) => {
                let next_bytes_len = current_stream.len() as NumberBytesSize;
                let left_bytes_to_read = number_of_bytes_payload as usize - payload_as_bytes.len();
                let next_slice_upper_bound =
                    next_bytes_len.min(left_bytes_to_read as NumberBytesSize) as usize;
                let next_bytes = &current_stream[0..next_slice_upper_bound];
                current_stream = &current_stream[next_slice_upper_bound..];
                payload_as_bytes.extend_from_slice(next_bytes);
                if payload_as_bytes.len() == number_of_bytes_payload as usize {
                    return ByteStreamReadingState::Done {
                        parsed: ParsedBinaryStream {
                            number_of_bytes_payload,
                            type_tag,
                            payload_as_bytes,
                        },
                        rest: current_stream,
                    };
                }

                ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                    number_of_bytes_payload,
                    type_tag,
                    payload_as_bytes,
                })
            }
            ByteStreamReadingState::Done { parsed, rest } => {
                return ByteStreamReadingState::Done { parsed, rest };
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::binary_parsing::ParsedBinaryStream;

    use super::ByteStreamReadingState;

    #[test]
    fn parse_empty_at_start() {
        let input: &[u8] = &[];
        let actual = ByteStreamReadingState::start(input);
        let expected = ByteStreamReadingState::Length {
            shift_counter: 0,
            current_number_of_bytes: 0,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_number_of_length_of_byte_stream() {
        fn assert_case(input: &[u8], expected: ByteStreamReadingState) {
            let actual = ByteStreamReadingState::start(input);
            assert_eq!(expected, actual, "Input {:?}\n", input);
        }
        // 000_0010 | 000_0010
        // 00000_001 0000_0010
        assert_case(
            &[0b1000_0010, 0b1000_0010],
            ByteStreamReadingState::Length {
                shift_counter: 2,
                current_number_of_bytes: 258,
            },
        );
        assert_case(
            &[0b0001_0101],
            ByteStreamReadingState::TypeTag {
                number_of_bytes_payload: 21,
                wip_type_tag: 0,
                number_of_read_tap_type_bytes: 0,
            },
        );
        assert_case(
            &[0b1001_0101],
            ByteStreamReadingState::Length {
                shift_counter: 1,
                current_number_of_bytes: 21,
            },
        );
        assert_case(
            &[0b0001_0101, 0b0101],
            ByteStreamReadingState::TypeTag {
                number_of_bytes_payload: 21,
                wip_type_tag: 5,
                number_of_read_tap_type_bytes: 1,
            },
        );
        assert_case(
            &[0b0001_0101, 0b0, 0b1],
            ByteStreamReadingState::PayLoad(ParsedBinaryStream {
                number_of_bytes_payload: 21,
                type_tag: 256,
                payload_as_bytes: Vec::new(),
            }),
        );
        assert_case(
            &[
                // number of bytes
                0b0000_0011,
                // end of number of payload bytes
                // tag
                0b011,
                0b0,
                // payload
                0b1,
                0b11,
                0b111,
                // rest
                0b1111_1111,
            ],
            ByteStreamReadingState::Done {
                parsed: ParsedBinaryStream {
                    number_of_bytes_payload: 3,
                    type_tag: 3,
                    payload_as_bytes: vec![0b1, 0b11, 0b111],
                },
                rest: &[0b1111_1111],
            },
        );
    }
}

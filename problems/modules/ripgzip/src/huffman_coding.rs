#![forbid(unsafe_code)]

use std::{char::MAX, collections::HashMap, convert::TryFrom, fmt::Error, hash::Hash, io::BufRead};

use anyhow::{anyhow, ensure, Context, Result};
use log::debug;

use crate::bit_reader::{BitReader, BitSequence};

////////////////////////////////////////////////////////////////////////////////

static LENGS_PERM: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

static MAXCODES: usize = 286;

pub fn decode_litlen_distance_trees<T: BufRead>(
    bit_reader: &mut BitReader<T>,
) -> Result<(HuffmanCoding<LitLenToken>, HuffmanCoding<DistanceToken>)> {
    // See RFC 1951, section 3.2.7.
    // TODO: your code goes here.
    let hlit = bit_reader.read_bits(5)?.bits() as usize + 257;
    let hdist = bit_reader.read_bits(5)?.bits() as usize + 1;
    let hclen = bit_reader.read_bits(4)?.bits() as usize + 4;
    debug!(
        "decode_litlen_distance_trees HLIT {}, HDIST {}, HCLEN {}",
        hlit, hdist, hclen,
    );
    let mut hclen_arr: Vec<u8> = (0..LENGS_PERM.len()).map(|x| 0 as u8).collect::<Vec<u8>>();
    for i in 0..hclen {
        hclen_arr[LENGS_PERM[i]] = bit_reader.read_bits(3)?.bits() as u8;
    }

    debug!("decode_litlen_distance_trees() : {:?}", hclen_arr);
    // Dyn tree Huffman code lengths
    let codes_4_tree = HuffmanCoding::<TreeCodeToken>::from_lengths(&hclen_arr[0..])?;

    let mut lit_len_arr: Vec<u8> = vec![0; hlit];
    let mut dist_arr: Vec<u8> = vec![0; hdist];

    let mut i = 0;
    while i < lit_len_arr.len() {
        let litlen = codes_4_tree.read_symbol(bit_reader)?;
        match litlen {
            TreeCodeToken::Length(x) => {
                lit_len_arr[i] = x;
                // debug!("decode_litlen_distance_trees(): litlen {} {:?}", i, x);
                i += 1;
            }
            TreeCodeToken::CopyPrev => {
                let repeat_len = bit_reader.read_bits(2)?.bits() as usize + 3;
                for it in i..i + repeat_len {
                    lit_len_arr[it] = lit_len_arr[i - 1];
                }
                // debug!(
                //     "decode_litlen_distance_trees(): repeat {} {} {:?}",
                //     i,
                //     lit_len_arr[i], repeat_len
                // );
                i += repeat_len as usize;
            }
            TreeCodeToken::RepeatZero { base, extra_bits } => {
                let repeat_len = bit_reader.read_bits(extra_bits)?.bits() as usize + base as usize;
                // debug!("decode_litlen_distance_trees(): zeros {:?}", repeat_len);
                i += repeat_len;
            }
        };
    }
    debug!("decode_litlen_distance_trees(): !!!!!!!!!!!!!!");
    let mut i = 0;
    while i < dist_arr.len() {
        let litlen = codes_4_tree.read_symbol(bit_reader)?;
        match litlen {
            TreeCodeToken::Length(x) => {
                dist_arr[i] = x;
                debug!("decode_litlen_distance_trees(): litlen {} {:?}", i, x);
                i += 1;
            }
            TreeCodeToken::CopyPrev => {
                let repeat_len = bit_reader.read_bits(2)?.bits() as usize + 3;
                for it in i..i + repeat_len {
                    dist_arr[it] = dist_arr[i - 1];
                }
                debug!("decode_litlen_distance_trees(): repeat {:?}", repeat_len);
                i += repeat_len;
            }
            TreeCodeToken::RepeatZero { base, extra_bits } => {
                let repeat_len = bit_reader.read_bits(extra_bits)?.bits() as usize + base as usize;
                debug!("decode_litlen_distance_trees(): zeros {:?}", repeat_len);
                i += repeat_len;
            }
        };
    }
    debug!("decode_litlen_distance_trees(): litlen {:?}", lit_len_arr);
    debug!("decode_litlen_distance_trees(): dist {:?}", dist_arr);

    Ok((
        HuffmanCoding::from_lengths(&lit_len_arr[0..])?,
        HuffmanCoding::from_lengths(&dist_arr[0..])?,
    ))
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub enum TreeCodeToken {
    Length(u8),
    CopyPrev,
    RepeatZero { base: u16, extra_bits: u8 },
}

impl TryFrom<HuffmanCodeWord> for TreeCodeToken {
    type Error = anyhow::Error;

    fn try_from(value: HuffmanCodeWord) -> Result<Self> {
        // See RFC 1951, section 3.2.7.
        // TODO: your code goes here.
        match value.0 {
            0..=15 => Ok(TreeCodeToken::Length(value.0 as u8)),
            16 => Ok(TreeCodeToken::CopyPrev),
            17 => Ok(TreeCodeToken::RepeatZero {
                base: 3,
                extra_bits: 3,
            }),
            18 => Ok(TreeCodeToken::RepeatZero {
                base: 11,
                extra_bits: 7,
            }),
            _ => Err(anyhow!(
                "try_from(): This is an incorrect code {}.",
                value.0,
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LitLenToken {
    Literal(u8),
    EndOfBlock,
    Length { base: u16, extra_bits: u8 },
}

impl TryFrom<HuffmanCodeWord> for LitLenToken {
    type Error = anyhow::Error;

    fn try_from(value: HuffmanCodeWord) -> Result<Self> {
        // See RFC 1951, section 3.2.5.
        // TODO: your code goes here.
        match value.0 {
            0..=255 => Ok(LitLenToken::Literal(value.0 as u8)),
            256 => Ok(LitLenToken::EndOfBlock),
            257..=264 => Ok(LitLenToken::Length {
                base: value.0 - 254,
                extra_bits: 0,
            }),
            265..=268 => Ok(LitLenToken::Length {
                base: 11 + (value.0 - 265) * 2,
                extra_bits: 1,
            }),
            269..=272 => Ok(LitLenToken::Length {
                base: 19 + (value.0 - 269) * 4,
                extra_bits: 2,
            }),
            273..=276 => Ok(LitLenToken::Length {
                base: 35 + (value.0 - 273) * 8,
                extra_bits: 3,
            }),
            277..=280 => Ok(LitLenToken::Length {
                base: 67 + (value.0 - 277) * 16,
                extra_bits: 4,
            }),
            281..=284 => Ok(LitLenToken::Length {
                base: 131 + (value.0 - 281) * 32,
                extra_bits: 5,
            }),
            285 => Ok(LitLenToken::Length {
                base: 258,
                extra_bits: 0,
            }),
            _ => Err(anyhow!(
                "try_from(): This is an incorrect code {}.",
                value.0,
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub struct DistanceToken {
    pub base: u16,
    pub extra_bits: u8,
}

impl TryFrom<HuffmanCodeWord> for DistanceToken {
    type Error = anyhow::Error;

    fn try_from(value: HuffmanCodeWord) -> Result<Self> {
        // See RFC 1951, section 3.2.5.
        // TODO: your code goes here.
        match value.0 {
            0..=3 => Ok(DistanceToken {
                base: value.0 + 1,
                extra_bits: 0,
            }),
            4..=5 => Ok(DistanceToken {
                base: 5 + (value.0 - 4) * 2,
                extra_bits: 1,
            }),
            6..=7 => Ok(DistanceToken {
                base: 9 + (value.0 - 6) * 4,
                extra_bits: 2,
            }),
            8..=9 => Ok(DistanceToken {
                base: 17 + (value.0 - 8) * 8,
                extra_bits: 3,
            }),
            10..=11 => Ok(DistanceToken {
                base: 33 + (value.0 - 10) * 16,
                extra_bits: 4,
            }),
            12..=13 => Ok(DistanceToken {
                base: 65 + (value.0 - 12) * 32,
                extra_bits: 5,
            }),
            14..=15 => Ok(DistanceToken {
                base: 129 + (value.0 - 14) * 64,
                extra_bits: 6,
            }),
            16..=17 => Ok(DistanceToken {
                base: 257 + (value.0 - 16) * 128,
                extra_bits: 7,
            }),
            18..=19 => Ok(DistanceToken {
                base: 513 + (value.0 - 18) * 256,
                extra_bits: 8,
            }),
            20..=21 => Ok(DistanceToken {
                base: 1025 + (value.0 - 20) * 512,
                extra_bits: 9,
            }),
            22..=23 => Ok(DistanceToken {
                base: 2049 + (value.0 - 22) * 1024,
                extra_bits: 10,
            }),
            24..=25 => Ok(DistanceToken {
                base: 4097 + (value.0 - 24) * 2048,
                extra_bits: 11,
            }),
            26..=27 => Ok(DistanceToken {
                base: 8193 + (value.0 - 26) * 4096,
                extra_bits: 12,
            }),
            28..=29 => Ok(DistanceToken {
                base: 16385 + (value.0 - 28) * 8192,
                extra_bits: 13,
            }),
            _ => Err(anyhow!(
                "try_from(): This is an incorrect code {}.",
                value.0,
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

const MAX_BITS: usize = 15;

#[derive(Debug)]
pub struct HuffmanCodeWord(pub u16);

pub struct HuffmanCoding<T> {
    pub map: HashMap<BitSequence, T>,
}

impl<T> HuffmanCoding<T>
where
    T: Copy + TryFrom<HuffmanCodeWord, Error = anyhow::Error>,
{
    pub fn new(map: HashMap<BitSequence, T>) -> Self {
        Self { map: map }
    }

    #[allow(unused)]
    pub fn decode_symbol(&self, seq: BitSequence) -> Option<T> {
        // TODO: your code goes here.
        self.map.get(&seq).map(|v| *v)
    }

    pub fn read_symbol<U: BufRead>(&self, bit_reader: &mut BitReader<U>) -> Result<T> {
        // // TODO: your code goes here.
        // let mut bits = bit_reader.read_bits(1)?;
        let mut bits: BitSequence = BitSequence::new(0, 0);
        for _bits_number in 0..=MAX_BITS {
            let iter_bits = bit_reader.read_bits(1)?;
            // debug!(
            //     "read_symbol before concat iter_bits {:?} bits {:?}",
            //     iter_bits, bits
            // );
            bits = bits.concat(iter_bits);
            let symbol = self.map.get(&bits);
            if let Some(val) = symbol {
                debug!(
                    "read_symbol mapped a symbol {:#b} len {}",
                    bits.bits(),
                    bits.len()
                );
                return Ok(*val);
            };
        }
        // debug!("read_symbol mapped a symbol {:#b}", bits.bits());

        Err(anyhow::Error::msg(
            "read_symbol(): Can not find a symbol in the corresponding Huffman tree.",
        ))
    }

    pub fn from_lengths(code_lengths: &[u8]) -> Result<Self> {
        // See RFC 1951, section 3.2.2.
        // TODO: your code goes here.
        let mut code: u16 = 0;
        let mut bl_count: Vec<u16> = vec![0; code_lengths.len() + 1];
        let mut next_code: Vec<u16> = vec![0; code_lengths.len() + 1];
        let mut codes: Vec<u16> = vec![0; code_lengths.len() + 1];
        let mut map: HashMap<BitSequence, T> = HashMap::new();

        bl_count[0] = 0;
        for len in code_lengths.iter() {
            if *len == 0 {
                continue;
            }
            bl_count[*len as usize] += 1;
        }
        for bits in 1..=code_lengths.len() {
            code = (code + bl_count[bits - 1]) << 1;
            next_code[bits] = code;
        }
        for n in 0..code_lengths.len() {
            let len = code_lengths[n];
            codes[n] = next_code[len as usize];
            next_code[len as usize] += 1;
        }
        let mut enumer = 0;

        for e in 0..code_lengths.len() {
            let w = HuffmanCodeWord(e as u16);
            map.insert(
                BitSequence::new(codes[e], code_lengths[e]),
                T::try_from(w).unwrap(),
            );
            // println!(
            //     "from_lengths map k {:?} v {:?}",
            //     BitSequence::new(codes[e], code_lengths[e]),
            //     enumer
            // );
            enumer += 1;
        }
        Ok(Self { map: map })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Value(u16);

    impl TryFrom<HuffmanCodeWord> for Value {
        type Error = anyhow::Error;

        fn try_from(x: HuffmanCodeWord) -> Result<Self> {
            Ok(Self(x.0))
        }
    }

    #[test]
    fn from_lengths() -> Result<()> {
        let code = HuffmanCoding::<Value>::from_lengths(&[2, 3, 4, 3, 3, 4, 2])?;

        assert_eq!(
            code.decode_symbol(BitSequence::new(0b00, 2)),
            Some(Value(0)),
        );
        assert_eq!(
            code.decode_symbol(BitSequence::new(0b100, 3)),
            Some(Value(1)),
        );
        assert_eq!(
            code.decode_symbol(BitSequence::new(0b1110, 4)),
            Some(Value(2)),
        );
        assert_eq!(
            code.decode_symbol(BitSequence::new(0b101, 3)),
            Some(Value(3)),
        );
        assert_eq!(
            code.decode_symbol(BitSequence::new(0b110, 3)),
            Some(Value(4)),
        );
        assert_eq!(
            code.decode_symbol(BitSequence::new(0b1111, 4)),
            Some(Value(5)),
        );
        assert_eq!(
            code.decode_symbol(BitSequence::new(0b01, 2)),
            Some(Value(6)),
        );

        assert_eq!(code.decode_symbol(BitSequence::new(0b0, 1)), None);
        assert_eq!(code.decode_symbol(BitSequence::new(0b10, 2)), None);
        assert_eq!(code.decode_symbol(BitSequence::new(0b111, 3)), None,);

        Ok(())
    }

    #[test]
    fn read_symbol() -> Result<()> {
        let code = HuffmanCoding::<Value>::from_lengths(&[2, 3, 4, 3, 3, 4, 2])?;
        let mut data: &[u8] = &[0b10111001, 0b11001010, 0b11101101];
        let mut reader = BitReader::new(&mut data);

        assert_eq!(code.read_symbol(&mut reader)?, Value(1));
        assert_eq!(code.read_symbol(&mut reader)?, Value(2));
        assert_eq!(code.read_symbol(&mut reader)?, Value(3));
        assert_eq!(code.read_symbol(&mut reader)?, Value(6));

        assert_eq!(code.read_symbol(&mut reader)?, Value(0));
        assert_eq!(code.read_symbol(&mut reader)?, Value(2));
        assert_eq!(code.read_symbol(&mut reader)?, Value(4));
        assert!(code.read_symbol(&mut reader).is_err());

        Ok(())
    }

    #[test]
    fn from_lengths_with_zeros() -> Result<()> {
        let lengths = [3, 4, 5, 5, 0, 0, 6, 6, 4, 0, 6, 0, 7];
        let code = HuffmanCoding::<Value>::from_lengths(&lengths)?;
        let mut data: &[u8] = &[
            0b00100000, 0b00100001, 0b00010101, 0b10010101, 0b00110101, 0b00011101,
        ];
        let mut reader = BitReader::new(&mut data);
        assert_eq!(code.read_symbol(&mut reader)?, Value(0));
        assert_eq!(code.read_symbol(&mut reader)?, Value(1));
        assert_eq!(code.read_symbol(&mut reader)?, Value(2));
        assert_eq!(code.read_symbol(&mut reader)?, Value(3));
        assert_eq!(code.read_symbol(&mut reader)?, Value(6));
        assert_eq!(code.read_symbol(&mut reader)?, Value(7));
        assert_eq!(code.read_symbol(&mut reader)?, Value(8));
        assert_eq!(code.read_symbol(&mut reader)?, Value(10));
        assert_eq!(code.read_symbol(&mut reader)?, Value(12));
        assert!(code.read_symbol(&mut reader).is_err());

        Ok(())
    }

    #[test]
    fn from_lengths_additional() -> Result<()> {
        let lengths = [
            9, 10, 10, 8, 8, 8, 5, 6, 4, 5, 4, 5, 4, 5, 4, 4, 5, 4, 4, 5, 4, 5, 4, 5, 5, 5, 4, 6, 6,
        ];
        let code = HuffmanCoding::<Value>::from_lengths(&lengths)?;
        let mut data: &[u8] = &[
            0b11111000, 0b10111100, 0b01010001, 0b11111111, 0b00110101, 0b11111001, 0b11011111,
            0b11100001, 0b01110111, 0b10011111, 0b10111111, 0b00110100, 0b10111010, 0b11111111,
            0b11111101, 0b10010100, 0b11001110, 0b01000011, 0b11100111, 0b00000010,
        ];
        let mut reader = BitReader::new(&mut data);

        assert_eq!(code.read_symbol(&mut reader)?, Value(10));
        assert_eq!(code.read_symbol(&mut reader)?, Value(7));
        assert_eq!(code.read_symbol(&mut reader)?, Value(27));
        assert_eq!(code.read_symbol(&mut reader)?, Value(22));
        assert_eq!(code.read_symbol(&mut reader)?, Value(9));
        assert_eq!(code.read_symbol(&mut reader)?, Value(0));
        assert_eq!(code.read_symbol(&mut reader)?, Value(11));
        assert_eq!(code.read_symbol(&mut reader)?, Value(15));
        assert_eq!(code.read_symbol(&mut reader)?, Value(2));
        assert_eq!(code.read_symbol(&mut reader)?, Value(20));
        assert_eq!(code.read_symbol(&mut reader)?, Value(8));
        assert_eq!(code.read_symbol(&mut reader)?, Value(4));
        assert_eq!(code.read_symbol(&mut reader)?, Value(23));
        assert_eq!(code.read_symbol(&mut reader)?, Value(24));
        assert_eq!(code.read_symbol(&mut reader)?, Value(5));
        assert_eq!(code.read_symbol(&mut reader)?, Value(26));
        assert_eq!(code.read_symbol(&mut reader)?, Value(18));
        assert_eq!(code.read_symbol(&mut reader)?, Value(12));
        assert_eq!(code.read_symbol(&mut reader)?, Value(25));
        assert_eq!(code.read_symbol(&mut reader)?, Value(1));
        assert_eq!(code.read_symbol(&mut reader)?, Value(3));
        assert_eq!(code.read_symbol(&mut reader)?, Value(6));
        assert_eq!(code.read_symbol(&mut reader)?, Value(13));
        assert_eq!(code.read_symbol(&mut reader)?, Value(14));
        assert_eq!(code.read_symbol(&mut reader)?, Value(16));
        assert_eq!(code.read_symbol(&mut reader)?, Value(17));
        assert_eq!(code.read_symbol(&mut reader)?, Value(19));
        assert_eq!(code.read_symbol(&mut reader)?, Value(21));

        Ok(())
    }
}

#![forbid(unsafe_code)]
#![allow(unused)]

use std::{
    convert::TryFrom,
    io::{BufRead, Write},
};

use anyhow::{anyhow, ensure, Context, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::huffman_coding::{self, LitLenToken};
use crate::tracking_writer::TrackingWriter;
use crate::{
    bit_reader::{BitReader, BitSequence},
    huffman_coding::DistanceToken,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct BlockHeader {
    pub is_final: bool,
    pub compression_type: CompressionType,
}

#[derive(Debug, PartialEq)]
pub enum CompressionType {
    Uncompressed = 0,
    FixedTree = 1,
    DynamicTree = 2,
    Reserved = 3,
}

impl From<BitSequence> for CompressionType {
    fn from(value: BitSequence) -> Self {
        match (value.bits(), value.len()) {
            (0, 2) => Self::Uncompressed,
            (1, 2) => Self::FixedTree,
            (2, 2) => Self::DynamicTree,
            _ => Self::Reserved,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DeflateReader<T> {
    bit_reader: BitReader<T>,
    // TODO: your code goes here.
    // lit_len_huffman_:huffman_coding::HuffmanCoding<LitLenToken>,
    // dist_token_huffman_: huffman_coding::HuffmanCoding<DistanceToken>,
}

impl<T: BufRead> DeflateReader<T> {
    pub fn new(bit_reader: BitReader<T>) -> Self {
        // TODO: your code goes here.
        Self {
            bit_reader: bit_reader,
        }
    }

    pub fn next_block(&mut self) -> Option<Result<(BlockHeader, &mut BitReader<T>)>> {
        // TODO: your code goes here.
        Some(Ok((
            BlockHeader {
                is_final: self.bit_reader.read_bits(1).unwrap()
                    == crate::bit_reader::BitSequence::new(0b1, 1),
                compression_type: CompressionType::from(self.bit_reader.read_bits(2).unwrap()),
            },
            &mut self.bit_reader,
        )))
    }
}

// TODO: your code goes here.

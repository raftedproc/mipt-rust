#![forbid(unsafe_code)]
#![allow(unused)]

use std::io::{self, BufRead};

use byteorder::ReadBytesExt;
use log::debug;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BitSequence {
    bits: u16,
    len: u8,
}

impl BitSequence {
    pub fn new(bits: u16, len: u8) -> Self {
        // NB: make sure to zero unused bits so that Eq and Hash work as expected.
        // TODO: your code goes here.
        Self {
            bits: bits,
            len: len,
        }
    }

    pub fn bits(&self) -> u16 {
        // TODO: your code goes here.
        self.bits
    }

    pub fn len(&self) -> u8 {
        // TODO: your code goes here.
        self.len
    }

    pub fn concat(self, other: Self) -> Self {
        // TODO: your code goes here.
        Self {
            bits: self.bits << other.len | other.bits,
            len: other.len + self.len,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct BitReader<T> {
    stream: T,
    // TODO: your code goes here.
    bits_left_: u8,
    buf_: u16,
}

impl<T: BufRead> BitReader<T> {
    pub fn new(stream: T) -> Self {
        // TODO: your code goes here.
        Self {
            stream: stream,
            bits_left_: 0,
            buf_: 0,
        }
    }

    pub fn get_mask(&self, len: u8) -> u16 {
        match len {
            16 => 0b1111111111111111,
            15 => 0b0111111111111111,
            14 => 0b0011111111111111,
            13 => 0b0001111111111111,
            12 => 0b0000111111111111,
            11 => 0b0000011111111111,
            10 => 0b0000001111111111,
            9 => 0b0000000111111111,
            8 => 0b0000000011111111,
            7 => 0b0000000001111111,
            6 => 0b0000000000111111,
            5 => 0b0000000000011111,
            4 => 0b0000000000001111,
            3 => 0b0000000000000111,
            2 => 0b0000000000000011,
            1 => 0b0000000000000001,
            _ => 0x0,
        }
    }

    pub fn read_bits(&mut self, len: u8) -> io::Result<BitSequence> {
        // TODO: your code goes here.
        // println!(
        //     "read_bits read 1 len {} bits_left {} buf {:#b}",
        //     len, self.bits_left_, self.buf_
        // );
        if len == 0 {
            // println! {"read_bits(): early quit"}
            return Ok(BitSequence::new(0, 0));
        }
        if self.bits_left_ >= len {
            let mask = self.get_mask(len);
            let res = Ok(BitSequence::new((self.buf_ & mask).into(), len));
            self.bits_left_ -= len;
            self.buf_ >>= len;
            return res;
        }
        // let res = BitSequence::new((self.buf_).into(), self.bits_left_);
        let to_read_more = len - self.bits_left_;
        let mask = self.get_mask(len);
        // println!("read_bits to read more {} bits", to_read_more);
        match to_read_more {
            9..=16 => {
                let read_1st_byte = self.stream.read_u8()? as u16;
                let result_bits = self.buf_ | read_1st_byte << self.bits_left_;
                let sign_bits_in_2nd_byte = to_read_more - 8;
                let mask_4_2nd_byte = self.get_mask(sign_bits_in_2nd_byte);
                let read_2nd_byte = self.stream.read_u8()? as u16;
                let result_bits =
                    result_bits | (read_2nd_byte & mask_4_2nd_byte) << (8 + self.bits_left_);
                self.buf_ = read_2nd_byte >> sign_bits_in_2nd_byte;
                // println!(
                //     "read_bits 1stbyte {:#b} 2ndbyte {:#b} concat {:#b} left {:#b}",
                //     read_1st_byte, read_2nd_byte, result_bits, self.buf_
                // );
                // println!(
                //     "read_bits len {} bits_left {} buf {:#b}",
                //     len, self.bits_left_, self.buf_
                // );
                self.bits_left_ = 16 + self.bits_left_ - len;
                return Ok(BitSequence::new((result_bits).into(), len));
            }
            0..=8 => {
                let read_1st_byte = self.stream.read_u8()?;
                // println!(
                //     "read_bits 1stbyte {:#b} self.buf_ {:#b} concat {:#b}  bits_left {}",
                //     read_1st_byte,
                //     self.buf_,
                //     self.buf_ | (read_1st_byte as u16) << self.bits_left_,
                //     self.bits_left_,
                // );
                self.buf_ = self.buf_ | (read_1st_byte as u16) << self.bits_left_;
                // debug!(
                //     "read_bits 1stbyte {:#b} concat {:#b}",
                //     read_1st_byte, self.buf_
                // );
                self.bits_left_ = self.bits_left_ + 8 - len;
            }
            _ => {
                // debug!("Strange len");
                ()
            } // need to throw here
        }
        let res = BitSequence::new((self.buf_ & mask).into(), len);
        self.buf_ = self.buf_ >> len;

        Ok(res)
    }

    /// Discard all the unread bits in the current byte and return a mutable reference
    /// to the underlying reader.
    pub fn borrow_reader_from_boundary(&mut self) -> &mut T {
        // TODO: your code goes here.
        self.buf_ = 0;
        self.bits_left_ = 0;
        return &mut self.stream;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::ReadBytesExt;

    #[test]
    fn read_bits() -> io::Result<()> {
        let data: &[u8] = &[0b01100011, 0b11011011, 0b10101111];
        let mut reader = BitReader::new(data);
        assert_eq!(reader.read_bits(1)?, BitSequence::new(0b1, 1));
        assert_eq!(reader.read_bits(2)?, BitSequence::new(0b01, 2));
        assert_eq!(reader.read_bits(3)?, BitSequence::new(0b100, 3));
        assert_eq!(reader.read_bits(4)?, BitSequence::new(0b1101, 4));
        assert_eq!(reader.read_bits(5)?, BitSequence::new(0b10110, 5));
        assert_eq!(reader.read_bits(8)?, BitSequence::new(0b01011111, 8));
        assert_eq!(
            reader.read_bits(2).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );

        let mut reader = BitReader::new(data);
        assert_eq!(reader.read_bits(9)?, BitSequence::new(0b101100011, 9));
        assert_eq!(
            reader.read_bits(15)?,
            BitSequence::new(0b101011111101101, 15)
        );

        Ok(())
    }

    #[test]
    fn borrow_reader_from_boundary() -> io::Result<()> {
        let data: &[u8] = &[0b01100011, 0b11011011, 0b10101111];
        let mut reader = BitReader::new(data);
        assert_eq!(reader.read_bits(3)?, BitSequence::new(0b011, 3));
        assert_eq!(reader.borrow_reader_from_boundary().read_u8()?, 0b11011011);
        assert_eq!(reader.read_bits(8)?, BitSequence::new(0b10101111, 8));
        Ok(())
    }
}

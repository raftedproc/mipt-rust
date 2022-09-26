#![forbid(unsafe_code)]

use std::io::BufRead;

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use crc::Crc;
use log::debug;

use crate::{
    bit_reader::{BitReader, BitSequence},
    deflate::{BlockHeader, DeflateReader},
    // tracking_writer::TrackingWriter,
};

////////////////////////////////////////////////////////////////////////////////

const ID1: u8 = 0x1f;
const ID2: u8 = 0x8b;

const CM_DEFLATE: u8 = 8;

const FTEXT_OFFSET: u8 = 0;
const FHCRC_OFFSET: u8 = 1;
const FEXTRA_OFFSET: u8 = 2;
const FNAME_OFFSET: u8 = 3;
const FCOMMENT_OFFSET: u8 = 4;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MemberHeader {
    pub compression_method: CompressionMethod,
    pub modification_time: u32,
    pub extra: Option<Vec<u8>>,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub extra_flags: u8,
    pub os: u8,
    pub has_crc: bool,
    pub is_text: bool,
}

impl MemberHeader {
    pub fn crc16(&self) -> u16 {
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut digest = crc.digest();

        digest.update(&[ID1, ID2, self.compression_method.into(), self.flags().0]);
        digest.update(&self.modification_time.to_le_bytes());
        digest.update(&[self.extra_flags, self.os]);

        if let Some(extra) = &self.extra {
            digest.update(&(extra.len() as u16).to_le_bytes());
            digest.update(extra);
        }

        if let Some(name) = &self.name {
            digest.update(name.as_bytes());
            digest.update(&[0]);
        }

        if let Some(comment) = &self.comment {
            digest.update(comment.as_bytes());
            digest.update(&[0]);
        }

        (digest.finalize() & 0xffff) as u16
    }

    pub fn flags(&self) -> MemberFlags {
        let mut flags = MemberFlags(0);
        flags.set_is_text(self.is_text);
        flags.set_has_crc(self.has_crc);
        flags.set_has_extra(self.extra.is_some());
        flags.set_has_name(self.name.is_some());
        flags.set_has_comment(self.comment.is_some());
        flags
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub enum CompressionMethod {
    Deflate,
    Unknown(u8),
}

impl From<u8> for CompressionMethod {
    fn from(value: u8) -> Self {
        match value {
            CM_DEFLATE => Self::Deflate,
            x => Self::Unknown(x),
        }
    }
}

impl From<CompressionMethod> for u8 {
    fn from(method: CompressionMethod) -> u8 {
        match method {
            CompressionMethod::Deflate => CM_DEFLATE,
            CompressionMethod::Unknown(x) => x,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MemberFlags(u8);

#[allow(unused)]
impl MemberFlags {
    fn bit(&self, n: u8) -> bool {
        (self.0 >> n) & 1 != 0
    }

    fn set_bit(&mut self, n: u8, value: bool) {
        if value {
            self.0 |= 1 << n;
        } else {
            self.0 &= !(1 << n);
        }
    }

    pub fn is_text(&self) -> bool {
        self.bit(FTEXT_OFFSET)
    }

    pub fn set_is_text(&mut self, value: bool) {
        self.set_bit(FTEXT_OFFSET, value)
    }

    pub fn has_crc(&self) -> bool {
        self.bit(FHCRC_OFFSET)
    }

    pub fn set_has_crc(&mut self, value: bool) {
        self.set_bit(FHCRC_OFFSET, value)
    }

    pub fn has_extra(&self) -> bool {
        self.bit(FEXTRA_OFFSET)
    }

    pub fn set_has_extra(&mut self, value: bool) {
        self.set_bit(FEXTRA_OFFSET, value)
    }

    pub fn has_name(&self) -> bool {
        self.bit(FNAME_OFFSET)
    }

    pub fn set_has_name(&mut self, value: bool) {
        self.set_bit(FNAME_OFFSET, value)
    }

    pub fn has_comment(&self) -> bool {
        self.bit(FCOMMENT_OFFSET)
    }

    pub fn set_has_comment(&mut self, value: bool) {
        self.set_bit(FCOMMENT_OFFSET, value)
    }
}
impl Into<MemberFlags> for u8 {
    fn into(self) -> MemberFlags {
        MemberFlags(self)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MemberFooter {
    pub data_crc32: u32,
    pub data_size: u32,
}

////////////////////////////////////////////////////////////////////////////////

pub struct GzipReader<T> {
    reader: T,
}

impl<T: BufRead> GzipReader<T> {
    pub fn new(reader: T) -> Self {
        Self { reader }
    }

    pub fn reader_mut(&mut self) -> &mut T {
        &mut self.reader
    }

    pub fn read_header(&mut self) -> Result<(MemberHeader, MemberFlags)> {
        let buf = self.reader.fill_buf().unwrap();
        let (header, m_flags, bytes_read) = GzipReader::<T>::parse_header(buf)?;
        debug!(
            "modif time {:?} bytes_read {}",
            header.modification_time, bytes_read
        );
        self.reader.consume(bytes_read);
        Ok((header, m_flags))
    }

    pub fn ret_mem_reader(mut self) -> Result<MemberReader<T>> {
        return Ok(MemberReader { inner: self.reader });
    }

    pub fn parse_header(mut header: &[u8]) -> Result<(MemberHeader, MemberFlags, usize)> {
        // See RFC 1952, section 2.3.
        // TODO: your code goes here.
        // read |ID1|ID2|CM |FLG|     MTIME     |XFL|OS |
        if ID1 != header.read_u8()? {
            return Err(anyhow::Error::msg("err1"));
        }
        if ID2 != header.read_u8()? {
            return Err(anyhow::Error::msg("err2"));
        }
        let compression_method = CompressionMethod::from(header.read_u8()?);
        debug!("compression_method {:?}", compression_method);

        let member_flags: MemberFlags = header.read_u8()?.into();
        debug!("flags {:?}", member_flags);

        let modification_time = header.read_u32::<LittleEndian>()?;

        let extra_flags = header.read_u8()?;
        let os = header.read_u8()?;
        debug!("os {:?}", os);
        let mut read_bytes = 10;

        let extra: Option<Vec<u8>> = if member_flags.has_extra() {
            let xlen = header.read_u16::<LittleEndian>()?;
            // ??? the result is unknown
            read_bytes += 2;
            Some(header[..xlen as usize].to_vec())
        } else {
            None
        };
        debug!("extra {:?}", extra);

        // !!!! It is not clear if to_vec() exhaust input reader or not
        let name: Option<String> = if member_flags.has_name() {
            // !!!! default value is unknown
            let len = header
                .into_iter()
                .position(|&c| c == b'\0')
                .unwrap_or(header.len());
            read_bytes += len;
            let mut res = String::new();
            for &c in &header[..len] {
                res.push(char::from(c));
            }
            debug!("name {:?}", res);
            Some(res)
        } else {
            None
        };
        debug!("mf has comments set {:?}", member_flags.has_comment());
        let comment: Option<String> = if member_flags.has_comment() {
            let len = header
                .into_iter()
                .position(|&c| c == b'\0')
                .unwrap_or(header.len());
            read_bytes += len;
            let mut res = String::new();
            for &c in &header[..len] {
                res.push(char::from(c));
            }
            debug!("comment {:?}", res);
            Some(res)
        } else {
            None
        };
        let crc16: Option<u16> = if member_flags.has_crc() {
            read_bytes += 2;
            Some(header.read_u16::<LittleEndian>()?)
        } else {
            None
        };
        let member_header = MemberHeader {
            compression_method: compression_method,
            modification_time: modification_time,
            extra_flags: extra_flags,
            os: os,
            extra: extra,
            name: name,
            comment: comment,
            has_crc: member_flags.has_crc(),
            is_text: member_flags.is_text(),
        };

        match crc16 {
            Some(crc) => {
                if crc != member_header.crc16() {
                    return Err(anyhow::Error::msg("wrong crc 16"));
                }
            }
            None => (),
        };

        Ok((member_header, member_flags, read_bytes))
    }

    // TODO: your code goes here.
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeflateCompressionMethod {
    NoCompression,
    FixedHuffmanCodes,
    DynamicHuffmanCodes,
    ErrorReserved,
}

impl From<BitSequence> for DeflateCompressionMethod {
    fn from(value: BitSequence) -> Self {
        match (value.bits(), value.len()) {
            (0, 2) => Self::NoCompression,
            (1, 2) => Self::FixedHuffmanCodes,
            (2, 2) => Self::DynamicHuffmanCodes,
            _ => Self::ErrorReserved,
        }
    }
}

pub struct MemberReader<T> {
    pub inner: T,
    // pub deflate_reader_: DeflateReader<T>,
}

impl<T: BufRead> MemberReader<T> {
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    // This fn consumes 3 bits but there is no way to move the stream 3 bits future
    pub fn read_header(&mut self) -> Result<(bool, DeflateCompressionMethod)> {
        let mut bit_reader = BitReader::new(self.inner_mut());
        let b1 = bit_reader.read_bits(1)?;
        let b_final = b1 == BitSequence::new(0b1, 1);
        let b2 = bit_reader.read_bits(2)?;
        let compression = DeflateCompressionMethod::from(b2);
        // let compression = DeflateCompressionMethod::from(bit_reader.read_bits(2)?);
        // bit_reader reads a byte so leaving it here basically looses 5 bits out of a byte
        // These 5 bits might contain info of a fixed or dynamic Huffman trees.
        debug!(
            "MemberReader::read_header(): b_final {} compression {:?} b1 {:?} b2 {:?}",
            b_final, compression, b1, b2,
        );
        Ok((b_final, compression))
    }

    // pub fn consume(&mut self, consume: usize) {
    //     self.inner_mut().consume(consume);
    // }
    pub fn fill_buf(&mut self) -> Result<&[u8]> {
        Ok(self.inner_mut().fill_buf()?)
    }

    pub fn read_footer(mut self) -> Result<(MemberFooter, GzipReader<T>)> {
        // TODO: your code goes here.
        let data_crc32 = self.inner.read_u32::<LittleEndian>()?;
        let data_size = self.inner.read_u32::<LittleEndian>()?;
        Ok((
            MemberFooter {
                data_crc32: data_crc32,
                data_size: data_size,
            },
            GzipReader::new(self.inner),
        ))
    }
    pub fn get_member(mut self) -> Result<MemberReader<T>> {
        // TODO: your code goes here.
        Ok(MemberReader { inner: self.inner })
    }
}

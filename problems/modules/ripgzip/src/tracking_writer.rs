#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::io::{self, Write};

use anyhow::{anyhow, ensure, Result};
use crc::{Crc, Digest, CRC_32_AUTOSAR, CRC_32_CKSUM, CRC_32_ISO_HDLC};
use log::debug;

////////////////////////////////////////////////////////////////////////////////

const HISTORY_SIZE: usize = 32768;

pub struct TrackingWriter<T> {
    inner: T,
    // TODO: your code goes here.
    buf_: VecDeque<u8>,
    bytes_written_: usize,
    _crc_: Crc<u32>,
}

impl<T: Write> Write for TrackingWriter<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // TODO: your code goes here.
        let res = self.inner.write(&buf);
        match res {
            Err(e) => return Err(e),
            Ok(written) => {
                self.bytes_written_ += written;
                self.buf_.extend(&buf[..written]);
            }
        };
        res
    }

    fn flush(&mut self) -> io::Result<()> {
        // TODO: your code goes here.
        self.inner.flush()
    }
}

impl<T: Write> TrackingWriter<T> {
    pub fn new(inner: T) -> Self {
        // TODO: your code goes here.
        Self {
            inner: inner,
            buf_: VecDeque::with_capacity(HISTORY_SIZE),
            bytes_written_: 0,
            _crc_: Crc::<u32>::new(&CRC_32_ISO_HDLC),
        }
    }

    /// Write a sequence of `len` bytes written `dist` bytes ago.
    /// Distance can be less then length that forces to clone [buf-distance] slice multiple times and the last can be partial.
    pub fn write_previous(&mut self, dist: usize, len: usize) -> Result<()> {
        // TODO: your code goes here.
        if self.buf_.len() < dist {
            return Err(anyhow!("The buffer is too small for the distance"));
        }
        let buf_len = self.buf_.len();
        let buf_start_idx = buf_len - dist;
        let buf_end_idx = buf_start_idx + len;

        if (buf_end_idx > buf_len) {
            let buf_to_write: Vec<u8> = self.buf_.range(buf_len - dist..buf_len).copied().collect();
            let times = len / buf_to_write.len();
            let bytes_reminder = len - times * buf_to_write.len();
            debug!(
                "write_previous times {}, len {}, buf_to_write {:?}",
                times, len, buf_to_write
            );
            let mut res: Result<usize, std::io::Error>;
            for i in 0..times {
                res = self.inner.write(&buf_to_write.clone()[..]);
                match res {
                    Err(e) => {
                        return Err(anyhow!("Write error"));
                    }
                    Ok(written) => {
                        // for i in 0..times {
                        self.buf_.extend(buf_to_write.clone());
                        self.bytes_written_ += buf_to_write.len();
                        // }
                        // if written < len {
                        //     return Err(anyhow!("Writer doesn't have space"));
                        // }
                    }
                }
            }
            if bytes_reminder > 0 {
                res = self.inner.write(&buf_to_write.clone()[..bytes_reminder]);
                match res {
                    Err(e) => {
                        return Err(anyhow!("Write error"));
                    }
                    Ok(written) => {
                        // for i in 0..times {
                        self.buf_
                            .extend(buf_to_write.clone()[..bytes_reminder].iter());
                        self.bytes_written_ += bytes_reminder;
                        // }
                        // if written < len {
                        //     return Err(anyhow!("Writer doesn't have space"));
                        // }
                    }
                }
            }
            return Ok(());
        }
        let res = self
            .inner
            .write(&self.buf_.make_contiguous()[buf_start_idx..buf_end_idx]);
        match res {
            Err(e) => {
                return Err(anyhow!("Write error"));
            }
            Ok(written) => {
                let prev_written_bytes: Vec<u8> = self
                    .buf_
                    .range(buf_start_idx..buf_start_idx + written)
                    .copied()
                    .collect();
                self.buf_.extend(prev_written_bytes);
                self.bytes_written_ += written;

                if written < len {
                    return Err(anyhow!("Writer doesn't have space"));
                }
            }
        };
        Ok(())
    }

    pub fn byte_count(&self) -> usize {
        // TODO: your code goes here.
        self.bytes_written_
    }

    pub fn crc32(mut self) -> u32 {
        // TODO: your code goes here.
        self._crc_.checksum(&self.buf_.make_contiguous())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::WriteBytesExt;

    #[test]
    fn write() -> Result<()> {
        let mut buf: &mut [u8] = &mut [0u8; 10];
        let mut writer = TrackingWriter::new(&mut buf);

        assert_eq!(writer.write(&[1, 2, 3, 4])?, 4);
        assert_eq!(writer.byte_count(), 4);

        assert_eq!(writer.write(&[4, 8, 15, 16, 23])?, 5);
        assert_eq!(writer.byte_count(), 9);

        assert_eq!(writer.write(&[0, 0, 123])?, 1);
        assert_eq!(writer.byte_count(), 10);

        assert_eq!(writer.write(&[42, 124, 234, 27])?, 0);
        assert_eq!(writer.byte_count(), 10);
        assert_eq!(writer.crc32(), 2992191065);

        Ok(())
    }

    #[test]
    fn write_previous() -> Result<()> {
        let mut buf: &mut [u8] = &mut [0u8; 512];
        let mut writer = TrackingWriter::new(&mut buf);

        for i in 0..=255 {
            writer.write_u8(i)?;
        }

        writer.write_previous(192, 128)?;
        assert_eq!(writer.byte_count(), 384);

        assert!(writer.write_previous(10000, 20).is_err());
        assert_eq!(writer.byte_count(), 384);
        assert!(writer.write_previous(256, 256).is_err());
        assert_eq!(writer.byte_count(), 512);

        assert!(writer.write_previous(1, 1).is_err());
        assert_eq!(writer.byte_count(), 512);
        assert_eq!(writer.crc32(), 2733545866);

        Ok(())
    }
}

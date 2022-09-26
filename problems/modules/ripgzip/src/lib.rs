#![forbid(unsafe_code)]

use std::io::{BufRead, Write};

use crate::{
    gzip::{GzipReader, MemberFooter},
    huffman_coding::LitLenToken,
};
use anyhow::{Context, Result};
use bit_reader::BitReader;
use byteorder::{LittleEndian, ReadBytesExt};
use deflate::DeflateReader;
use gzip::{DeflateCompressionMethod, MemberReader};
use huffman_coding::decode_litlen_distance_trees;
use log::*;
use tracking_writer::TrackingWriter;
mod bit_reader;
mod deflate;
mod gzip;
mod huffman_coding;
mod tracking_writer;

pub fn decompress<R: BufRead, W: Write>(mut input: R, mut output: W) -> Result<()> {
    // TODO: your code goes here.
    let mut gzip_reader: GzipReader<R> = GzipReader::new(input);
    let mut tracking_writer = TrackingWriter::new(output);
    let (header, m_flags) = gzip_reader.read_header()?;
    let mut member_reader = MemberReader {
        inner: gzip_reader.reader_mut(),
    };
    // let mut member_reader: MemberReader<R>;
    // let mut member_reader: MemberReader<R>;
    // WIP maybe unused
    // let (mut b_final, mut comp) = member_reader.read_header()?;

    let r = BitReader::new(member_reader.inner_mut());
    let mut d = DeflateReader::new(r);
    let (mut header, mut br) = d.next_block().unwrap()?;

    // Read gzip member
    while true {
        // member_reader = gzip_reader.ret_mem_reader()?;
        // (b_final, comp) = member_reader.read_header()?;
        debug!(
            "decompress(): final {:?} compression {:?} ",
            header.is_final, header.compression_type
        );
        // block read loop
        if header.compression_type == deflate::CompressionType::Uncompressed {
            let r = br.borrow_reader_from_boundary();
            let mut member_reader = MemberReader { inner: r };
            let mut uncompressed_len =
                member_reader.inner_mut().read_u16::<LittleEndian>()? as usize;
            let _uncompressed_nlen = member_reader.inner_mut().read_u16::<LittleEndian>()? as usize;
            debug!(
                "decompress(): uncompressed_len {} _uncompressed_nlen {}",
                uncompressed_len, _uncompressed_nlen,
            );
            while let Ok(buf) = member_reader.fill_buf() {
                let bytes_to_write = std::cmp::min(buf.len(), uncompressed_len);
                uncompressed_len -= bytes_to_write;
                let _written = tracking_writer.write(&buf[..bytes_to_write])?;
                debug!(
                    "decompress(): _written {} uncompressed_len {}",
                    _written, uncompressed_len
                );

                member_reader.inner_mut().consume(bytes_to_write);
                if uncompressed_len == 0 {
                    break;
                }
            }

            if uncompressed_len > 0 {
                error!("decompress(): unexpected number of bytes found in an uncompressed deflate block actual {} expected {}", uncompressed_len, tracking_writer.byte_count());
                break;
            }
        } else if header.compression_type == deflate::CompressionType::DynamicTree {
            // while true {
            // read dynamic huffman tree
            let (lit_len_huffman, dist_huffman) = decode_litlen_distance_trees(br)?;
            let mut buf: Vec<u8> = Vec::with_capacity(10);
            loop {
                let litlen = lit_len_huffman.read_symbol(br);
                match litlen? {
                    LitLenToken::EndOfBlock => {
                        debug!("decompress eob");
                        break;
                    }
                    LitLenToken::Literal(v) => {
                        buf.push(v);
                        if buf.len() == 10 {
                            tracking_writer.write(&buf)?;
                            buf.clear();
                        }
                        debug!("decompress literal {:}", v as char);
                    }
                    LitLenToken::Length { base, extra_bits } => {
                        // debug!("decompress match 1 {:} {}", base, extra_bits);
                        let len = (base
                            + if extra_bits > 0 {
                                br.read_bits(extra_bits)?.bits()
                            } else {
                                0
                            }) as usize;
                        // debug!("decompress match 2 {:?}", dist_huffman.map);
                        let dist_token = dist_huffman.read_symbol(br)?;
                        debug!(
                            "decompress match 3 {:} {}",
                            dist_token.base, dist_token.extra_bits
                        );

                        let dist = (dist_token.base
                            + if dist_token.extra_bits > 0 {
                                br.read_bits(dist_token.extra_bits)?.bits()
                            } else {
                                0
                            }) as usize;
                        debug!(
                            "decompress() dist {:} len {:} wr byte_count {}",
                            dist,
                            len,
                            tracking_writer.byte_count()
                        );
                        if !buf.is_empty() {
                            tracking_writer.write(&buf)?;
                            buf.clear();
                        }

                        tracking_writer.write_previous(dist as usize, len as usize)?;
                    }
                };
            }
            // If buf has some data then write it

            // if header.is_final {
            //     break;
            // }
            // }
        } else {
            debug!("decompress(): no supported compress method yet");
            break;
        }

        if header.is_final {
            break;
        }
        (header, br) = d.next_block().unwrap()?;
        debug!(
            "decompress(): end of the loop header is_final {} comp {:?}",
            header.is_final, header.compression_type
        );
    }

    Ok(())
}

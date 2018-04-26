use tokio_io::codec::{Encoder, Decoder};
use protobuf;
use protobuf::Message;
use types::*;
use tokio::io;
use bytes::{BytesMut, BufMut, BigEndian};
use bytes::ByteOrder;
use byteorder::WriteBytesExt;
use std::io::Write;
use integer_encoding::{VarInt, VarIntReader, VarIntWriter};

#[derive(Debug)]
pub struct ABCICodec;

impl ABCICodec {
    pub fn new() -> ABCICodec {
        ABCICodec
    }
}
const MAX_MESSAGE_SIZE:u64 = 104857600; // 100MB

impl Decoder for ABCICodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        let length = buf.len();

        if length == 0 || length > MAX_MESSAGE_SIZE as usize {
            return Ok(None);
        }

        let varint:(i64,usize) = i64::decode_var(&buf[..]);

        let message = protobuf::core::parse_from_bytes(
            &buf[varint.1 .. (varint.0 as usize + varint.1)]);
        
        buf.split_to(length);

        Ok(message.ok())
    }
}

impl Encoder for ABCICodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        let mut msg_to_vec = Vec::new();
        msg.write_to_vec(&mut msg_to_vec).unwrap();

        let msg_len: i64 = msg_to_vec.len() as i64;
        let varint = i64::encode_var_vec(msg_len);
        
        let mut writer = buf.writer();

        writer.write(&varint).unwrap();
        writer.write(&msg_to_vec).unwrap();
        writer.write(b"\x04\x1a\0").unwrap();
        
        Ok(())
    }
}
use std::io;
use bytes::BytesMut;
use tokio_codec::{Decoder, Encoder};
use resp::{Value, Decoder as RespDecoder};
use std::io::BufReader;
use std::str;


pub struct RespCodec;

impl Encoder for RespCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, msg:Vec<u8>, buf:&mut BytesMut) -> io::Result<()> {
        buf.reserve(msg.len());
        buf.extend(msg);
        Ok(())
    }
}

impl Decoder for RespCodec {
    type Item = Value;
    type Error = io::Error;

    fn decode(&mut self, buf:&mut BytesMut) -> io::Result<Option<Value>> {
        let s = if let Some(n) = buf.iter().rposition(|b| *b == b'\n') {    // 获取右侧第一个\n的位置
            let client_query = buf.split_to(n+1);    // 以该位置将序列分为两个部分
            match str::from_utf8(client_query.as_ref()) {    // 转换为utf-8编码的字符串切片
                Ok(s) => s.to_string(),
                Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            }
        } else {
            return Ok(None)
        };

        if let Ok(v) = RespDecoder::new(&mut BufReader::new(s.as_bytes())).decode() {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}
use std::io::{Read, Write};
use std::net::{TcpStream};

use crate::{Result};
use crate::helpers::{misconfiguration};

use bson::{Document};
use bson::{doc};

pub fn to_writer<W, M>(message: &M, writer: &mut W) -> Result<()>
where
    W: Write,
    M: serde::Serialize,
{
    let serialized = bson::to_bson(&message)?;

    if let Some(it) = serialized.as_document() {
        it.to_writer(writer)?;
        return Ok(())
    }

    if let Some(it) = serialized.as_str() {
        let wrapper = doc! {
            it: {}
        };

        wrapper.to_writer(writer)?;
        return Ok(())
    }

    misconfiguration(&format!("Bad BSON > {:?}", &serialized.element_type()))
}

pub fn to_bytes<M>(message: &M) -> Result<Vec<u8>>
where
    M: serde::Serialize,
{
    let mut packet = vec![];
    to_writer(message, &mut packet)?;
    return Ok(packet)
}

pub fn from_reader<R, M>(reader: R) -> Result<M>
where
    R: Read,
    M: for<'de> serde::Deserialize<'de>,
{
    let it = Document::from_reader(reader)?;
    let message: M = bson::from_bson(it.into())?;
    Ok(message)
}

pub fn from_bytes<M>(bytes: &[u8]) -> Result<M>
where
    M: for<'de> serde::Deserialize<'de>,
{
    from_reader(bytes)
}

pub fn read_packet(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut data = vec![0u8; 4];

    stream.read_exact(&mut data)?;

    let length = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;

    data.resize(length, 0);
    stream.read_exact(&mut data[4..])?;

    Ok(data)
}

pub fn read_message<M>(stream: &mut TcpStream) -> Result<M>
where
    M: for<'de> serde::Deserialize<'de>
{
    let packet = read_packet(stream)?;
    from_bytes(&packet)
}


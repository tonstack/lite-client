use tl_proto::{TlRead, TlResult};

pub fn lossy_read<'tl, T: TlRead<'tl>>(packet: &'tl [u8], offset: &mut usize) -> TlResult<Option<T>> {
    let orig_offset = *offset;
    let result = T::read_from(packet, offset);
    if let Ok(x) = result {
        Ok(Some(x))
    } else {
        *offset = orig_offset;
        Ok(None)
    }
}

pub fn fmt_string(bytes: &[u8], f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(
        f,
        "{}",
        std::string::String::from_utf8(bytes.to_vec()).unwrap()
    )
}

pub fn fmt_bytes(bytes: &[u8], f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(f, "0x{}", hex::encode(bytes))
}

pub mod struct_as_bytes {
    use tl_proto::{TlPacket, TlRead, TlResult, TlWrite};

    pub fn size_hint<T: TlWrite>(v: &T) -> usize {
        tl_proto::serialize(v).len()
    }

    pub fn write<P: TlPacket, T: TlWrite>(v: &T, packet: &mut P) {
        tl_proto::serialize(v).write_to(packet)
    }

    pub fn read<'tl, T: TlRead<'tl>>(packet: &'tl [u8], offset: &mut usize) -> TlResult<T> {
        <&'tl [u8]>::read_from(packet, offset).and_then(|x| tl_proto::deserialize(x))
    }
}
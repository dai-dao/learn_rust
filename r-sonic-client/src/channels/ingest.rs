use super::{Channel, ChannelMode, SonicStream};
use std::net::{ToSocketAddrs};
use crate::result::Result;



#[derive(Debug)]
pub struct IngestChannel {
    stream: SonicStream,
}


impl Channel for IngestChannel {
    fn start<A: ToSocketAddrs, S: ToString>(addr: A, password: S) -> Result<Self> {
        let stream = SonicStream::connect_with_start(ChannelMode::Ingest, addr, password)?;
        let out = IngestChannel { stream };
        Ok( out )
    }
}


impl IngestChannel {

    pub fn test_macro() {
        println!("{:?}", add!(1,2));
        println!("{:?}", add!(1,2,3));
        println!("{:?}", add_as!(1, 2, 3, 4))
    }

    pub fn push<S: ToString>(&self, collection: S, bucket: S, obj_id: S, content: S) -> Result<String> {
        Ok("ok".to_string())
    }
}
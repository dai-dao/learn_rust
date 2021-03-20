use super::{Channel, ChannelMode, SonicStream};
use std::net::{ToSocketAddrs};
use crate::result::*;


pub struct SearchChannel {
    stream: SonicStream,
}


impl Channel for SearchChannel {
    fn start<A: ToSocketAddrs, S: ToString>(addr: A, password: S) -> Result<Self> {
        let stream = SonicStream::connect_with_start(ChannelMode::Search, addr, password);
        Ok(SearchChannel { stream: stream.unwrap() })
    }
}
use super::{Channel, ChannelMode, SonicStream};
use std::net::{ToSocketAddrs};
use crate::result::*;
use crate::commands::{QuitCommand};


pub struct SearchChannel {
    stream: SonicStream,
}


impl Channel for SearchChannel {
    fn start<A: ToSocketAddrs, S: ToString>(addr: A, password: S) -> Result<Self> {
        let stream = SonicStream::connect_with_start(ChannelMode::Search, addr, password);
        Ok(SearchChannel { stream: stream.unwrap() })
    }
}

impl SearchChannel {
    init_command!( use QuitCommand for fn quit());
}
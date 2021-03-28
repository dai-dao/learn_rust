use super::{Channel, ChannelMode, SonicStream};
use std::net::{ToSocketAddrs};
use crate::result::Result;

use crate::commands::{QuitCommand, PushCommand, PingCommand, FlushCommand};


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

    init_command!( use QuitCommand for fn quit());
    init_command!( use PushCommand for fn push<'a>(collection: &'a str, 
                                                bucket: &'a str, 
                                                obj_id: &'a str, 
                                                content: &'a str,));
    init_command!( use PingCommand for fn ping());

    init_command!( use FlushCommand for fn flushc<'a>(collection: &'a str,));
    init_command!( use FlushCommand for fn flushb<'a>(collection: &'a str, 
                                                    bucket: &'a str => Some(bucket),));

    // pub fn test_macro() {
    //     println!("{:?}", add!(1,2));
    //     println!("{:?}", add!(1,2,3));
    //     println!("{:?}", add_as!(1, 2, 3, 4));

    //     make_public!{
    //         #[derive(Debug)]
    //         struct Name{
    //             n:i64,
    //             t:i64,
    //             g:i64,
    //         }
    //     }
    // }
}
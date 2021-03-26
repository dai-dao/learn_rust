use super::StreamCommand;
use crate::result::*;
use crate::channels::ChannelMode;
use regex::Regex;


const RE_START_RECEIVED_MESSAGE: &str = r"(?x)
    STARTED
    \s # started with mode
    (?P<mode>search|ingest|control)
    \s # which protocol used
    protocol\((?P<protocol>\d+)\)
    \s # maximum buffer size
    buffer\((?P<buffer_size>\d+)\)
";



#[derive(Debug)]
pub struct StartCommand {
    pub mode: ChannelMode,
    pub password: String,
}


pub struct StartCommandResponse {
    pub mode: ChannelMode,
    pub max_buffer_size: usize,
    pub protocol_version: usize
}


impl StreamCommand for StartCommand {

    type Response = StartCommandResponse;

    fn message(&self) -> String {
        let out = format!("START {} {}\r\n", self.mode, self.password);
        return out
    }

    fn receive(&self, response: String) -> Result<Self::Response> {
        dbg!(&response);

        // lazy_static! {
        //     static ref RE: Regex = Regex::new(RE_START_RECEIVED_MESSAGE).unwrap();
        // }
        let RE : Regex = Regex::new(RE_START_RECEIVED_MESSAGE).unwrap();

        if let Some(caps) = RE.captures(&response) {
            if self.mode.to_str() != &caps["mode"] {
                Err(Error::new(ErrorKind::SwitchMode))
            } else {
                let protocol_version : usize = caps["protocol"].parse()
                                            .expect("Must be digit by regex");
                let max_buffer_size : usize = caps["buffer_size"].parse()
                                            .expect("Must be digit by regex");
                Ok(StartCommandResponse {
                    mode: self.mode,
                    protocol_version: protocol_version,
                    max_buffer_size: max_buffer_size
                })
            }
        } else {
            Err(Error::new(ErrorKind::SwitchMode))
        }
    }
}

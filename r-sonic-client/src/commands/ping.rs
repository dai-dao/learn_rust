use super::StreamCommand;
use crate::result::*;


#[derive(Debug, Default)]
pub struct PingCommand {

}


impl StreamCommand for PingCommand {

    type Response = bool;

    fn message(&self) -> String {
        String::from("PING\r\n")
    }

    fn receive(&self, response: String) -> Result<Self::Response> {
        if response == "PONG\r\n" {
            Ok(true)
        } else {
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
use super::StreamCommand;
use crate::result::*;


#[derive(Debug, Default)]
pub struct QuitCommand {

}


impl StreamCommand for QuitCommand {

    type Response = bool;

    fn message(&self) -> String {
        String::from("QUIT\r\n")
    }

    fn receive(&self, response: String) -> Result<Self::Response> {
        if response.starts_with("ENDED ") {
            Ok(true)
        } else {
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
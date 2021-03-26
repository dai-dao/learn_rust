use super::StreamCommand;
use crate::result::*;


#[derive(Debug)]
pub struct PushCommand<'a> {
    pub collection: &'a str, 
    pub bucket: &'a str, 
    pub obj_id: &'a str, 
    pub content: &'a str
}


impl StreamCommand for PushCommand<'_> {

    type Response = bool;

    fn message(&self) -> String {
        let message = format!("PUSH {} {} {} \"{}\"\r\n",
                    self.collection, self.bucket, self.obj_id, self.content
        );
        dbg!(&message);
        message
    }

    fn receive(&self, response: String) -> Result<Self::Response> {
        dbg!(&response);

        if response == "OK\r\n" {
            Ok(true)
        } else {
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
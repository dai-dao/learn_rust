use super::StreamCommand;
use crate::result::*;


// Make bucket and obj_id as Option because it might or might not get passed into
// this struct
#[derive(Debug, Default)]
pub struct FlushCommand<'a> {
    pub collection: &'a str, 
    pub bucket: Option<&'a str>, 
    pub obj_id: Option<&'a str>,
}


impl StreamCommand for FlushCommand<'_> {

    type Response = usize;

    fn message(&self) -> String {
        // this catches which variants of the function to call
        let mut message = match(self.bucket, self.obj_id) {
            (Some(bucket), Some(obj_id)) => {
                format!("FLUSHO {} {} {}", self.collection, bucket, obj_id)
            }
            (Some(bucket), None) => format!("FLUSHB {} {}", self.collection, bucket),
            (None, None) => format!("FLUSHC {}", self.collection),
            _ => panic!("Invalid flush command"),
        };
        message.push_str("\r\n");
        message
    }

    fn receive(&self, response: String) -> Result<Self::Response> {
        if response.starts_with("RESULT ") {
            let count = response.split_whitespace().last().unwrap_or_default();
            count.parse().map_err(|_| {
                Error::new(ErrorKind::QueryResponseError(
                        "Can not parse count of flush method response to usize",))
            })
        } else {
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
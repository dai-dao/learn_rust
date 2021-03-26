mod quit;
pub use quit::*;

mod push;
pub use push::*;

mod start;
pub use start::*;

use crate::result::Result;


pub trait StreamCommand {
    type Response;
    const READ_LINES_COUNT : usize = 1;

    fn message(&self) -> String;
    fn receive(&self, response: String) -> Result<Self::Response>;
}
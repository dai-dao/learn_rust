use std::fmt;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::result::*;
use crate::commands::{StreamCommand, StartCommand};

mod search;
pub use search::*;

mod ingest;
pub use ingest::*;


const DEFAULT_SONIC_PROTOCOL_VERSION: usize = 1;
const UNINITIALIZED_MODE_MAX_BUFFER_SIZE: usize = 200;


// Channel modes supported by Sonic search backend.
#[derive(Debug, Clone, Copy)]
pub enum ChannelMode {
    Search,
    Ingest,
    Control
}

impl ChannelMode {
    pub fn to_str(&self) -> &str {
        match self{
            ChannelMode::Search => "search",
            ChannelMode::Ingest => "ingest",
            ChannelMode::Control => "control",
        }
    }
}

impl fmt::Display for ChannelMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.to_str())
    }
}


#[derive(Debug)]
pub struct SonicStream {
    stream: TcpStream,
    mode: Option<ChannelMode>,
    max_buffer_size: usize,
    protocol_version: usize,
}


impl SonicStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr)
                                    .map_err(|_| Error::new(ErrorKind::ConnectToServer))?;
        let channel = SonicStream { 
            stream, 
            mode: None,
            max_buffer_size: UNINITIALIZED_MODE_MAX_BUFFER_SIZE,
            protocol_version: DEFAULT_SONIC_PROTOCOL_VERSION,
        };
        // read
        let response = channel.read(1)?;
        if response.starts_with("CONNECTED") {
            Ok(channel)
        } else {
            Err(Error::new(ErrorKind::ConnectToServer))
        }
    }

    fn initiate<S: ToString>(&mut self, mode: ChannelMode, password: S) -> Result<()> {
        // make a start command here, then pass the command struct to write
        let command = StartCommand {
            mode: mode,
            password: password.to_string(),
        };
        let response = self.run_command(command)?;
        // update values from server response
        self.max_buffer_size = response.max_buffer_size;
        self.protocol_version = response.protocol_version;
        self.mode = Some(response.mode);

        Ok(())
    }

    fn write<SC: StreamCommand>(&mut self, command: &SC) -> Result<()> {
        let mut writer = BufWriter::with_capacity(200, &self.stream);
        let message = command.message();
        let _ = writer.write_all(message.as_bytes())
                    .map_err(|_| Error::new(ErrorKind::WriteStream))?;
        Ok(())
    }

    pub fn read(&self, max_read_lines: usize) -> Result<String> {
        let mut reader = BufReader::with_capacity(200, &self.stream);
        let mut response = String::new();
        let mut lines_read = 0;
        while lines_read < max_read_lines {
            reader.read_line(&mut response)
                  .map_err(|_| Error::new(ErrorKind::ReadStream))?;
            lines_read += 1;
        }
        Ok(response)
    }

    pub fn run_command<SC: StreamCommand>(&mut self, cmd: SC) -> Result<SC::Response> {
        self.write(&cmd);
        let response = self.read(SC::READ_LINES_COUNT)?;
        cmd.receive(response)
    }

    // used by Channels to initiate connection
    pub(crate) fn connect_with_start<A: ToSocketAddrs, S: ToString>(cm: ChannelMode, addr: A, password: S) -> Result<Self> {
        let mut channel = Self::connect(addr)?;
        channel.initiate(cm, password)?;
        Ok(channel)
    }
}


pub trait Channel {
    fn start<A: ToSocketAddrs, S: ToString>(addr: A, password: S) -> Result<Self> where Self: Sized;
}
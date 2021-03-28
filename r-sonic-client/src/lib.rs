#[macro_use]
mod macroses;

pub mod result;

mod channels;

mod commands;

pub use channels::*;


#[macro_use]
extern crate lazy_static;
extern crate regex;


#[cfg(test)]
mod tests {
    use crate::channels::Channel;
    use crate::channels::ChannelMode;
    use crate::channels::SonicStream;
    use crate::channels::SearchChannel;
    use crate::channels::IngestChannel;
    //
    use crate::result::*;

    #[test]
    fn format_channel_enums() {
        assert_eq!(format!("{}", ChannelMode::Search), "search");
        assert_eq!(format!("{}", ChannelMode::Ingest), "ingest");
        assert_eq!(format!("{}", ChannelMode::Control), "control");
    }
    #[test]
    fn test_tcp_connect() {
        let channel = SonicStream::connect("::1:1491");
        assert_eq!(channel.is_ok(), true);
    }

    #[test]
    fn test_search_channel() {
        let channel = SearchChannel::start("::1:1491", "SecretPassword");
        assert_eq!(channel.is_ok(), true);
        let out = channel.unwrap().quit();
        assert_eq!(out.is_ok(), true);
    }

    #[test]
    fn test_ingest_channel() {
        let channel = IngestChannel::start("::1:1491", "SecretPassword");
        assert_eq!(channel.is_ok(), true);
        let out = channel.unwrap().quit();
        assert_eq!(out.is_ok(), true);
    }

    #[test]
    fn test_ingest_query() -> Result<()> {
        let mut channel = IngestChannel::start("::1:1491", "SecretPassword")?;
        let pushed = channel.push("collection", "bucket", "object:1", "my really new good recipe")?;
        assert_eq!(pushed, true);
        let out = channel.quit()?;
        assert_eq!(out, true);
        Ok(())
    }

    #[test]
    fn test_ping() -> Result<()> {
        let mut channel = IngestChannel::start("::1:1491", "SecretPassword")?;
        let out = channel.ping()?;
        assert_eq!(out, true);
        Ok(())
    }

    #[test]
    fn test_flush() -> Result<()> {
        let mut channel = IngestChannel::start("::1:1491", "SecretPassword")?;
        let flushb_count = channel.flushb("collection1", "bucket")?;
        assert_eq!(flushb_count, 0);
        let flushc_count = channel.flushc("collection1")?;
        assert_eq!(flushc_count, 0);
        Ok(())
    }

    // #[test]
    // fn test_macro() {
    //     IngestChannel::test_macro()
    // }

    #[test]
    fn test_call() -> Result<()> {
        let mut channel = SearchChannel::start("::1:1491", "SecretPassword")?;
        channel.callCmd();

        // channel.callCmd();
        Ok(())
    }
}

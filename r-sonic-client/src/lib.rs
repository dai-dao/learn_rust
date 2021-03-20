#[macro_use]
mod macroses;

pub mod result;

mod channels;
pub use channels::*;


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
        println!("{}", format!("{}", ChannelMode::Search));
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
    }

    #[test]
    fn test_ingest_channel() {
        let channel = IngestChannel::start("::1:1491", "SecretPassword");
        assert_eq!(channel.is_ok(), true);
    }

    #[test]
    fn test_ingest_query() -> Result<()> {
        let channel = IngestChannel::start("::1:1491", "SecretPassword")?;
        let pushed = channel.push("collection", "bucket", "object:1", "my really new good recipe");
        assert_eq!(pushed.is_ok(), true);
        // dbg!(pushed);
        // channel.quit()?;
        Ok(())
    }

    #[test]
    fn test_macro() {
        IngestChannel::test_macro()
    }
}

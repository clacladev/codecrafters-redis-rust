const ID_PING: &str = "PING";

#[derive(Debug)]
pub enum InboundMessage {
    Ping,
}

impl TryFrom<&[u8]> for InboundMessage {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let message_string = String::from_utf8_lossy(value).to_string();
        let mut lines = message_string.lines();

        loop {
            let Some(line) = lines.next() else {
                anyhow::bail!(format!(
                    "-> Failed to parse inbound message '{}'",
                    message_string
                ))
            };
            let line = line.to_uppercase();
            match line.as_str() {
                ID_PING => return Ok(Self::Ping),
                _ => continue,
            }
        }
    }
}

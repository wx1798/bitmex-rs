mod announcement;
mod apikey;
mod chat;
mod order;

use transport::Transport;

#[derive(Clone)]
pub struct BitMEX {
    transport: Transport,
}

impl BitMEX {
    pub fn new() -> Self {
        BitMEX { transport: Transport::new() }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        BitMEX {
            transport: Transport::with_credential(api_key, api_secret),
        }
    }
}

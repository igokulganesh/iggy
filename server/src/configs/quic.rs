use byte_unit::Byte;
use iggy::utils::duration::IggyDuration;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuicConfig {
    pub enabled: bool,
    pub address: String,
    pub max_concurrent_bidi_streams: u64,
    pub datagram_send_buffer_size: Byte,
    pub initial_mtu: Byte,
    pub send_window: Byte,
    pub receive_window: Byte,
    #[serde_as(as = "DisplayFromStr")]
    pub keep_alive_interval: IggyDuration,
    #[serde_as(as = "DisplayFromStr")]
    pub max_idle_timeout: IggyDuration,
    pub certificate: QuicCertificateConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuicCertificateConfig {
    pub self_signed: bool,
    pub cert_file: String,
    pub key_file: String,
}

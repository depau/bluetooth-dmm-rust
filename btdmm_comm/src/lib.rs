pub use device::DmmDevice;
pub use device::scan_for_dmm;
pub use parser::DisplayValue;
pub use parser::Measurement;

#[derive(Debug, thiserror::Error)]
pub enum DmmError {
    #[error("Already connected")]
    AlreadyConnected,

    #[error("Device not connected")]
    NotConnected,

    #[error("Device disconnected")]
    DeviceDisconnected,

    #[error("Device not found")]
    DeviceNotFound,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

mod device;
mod parser;

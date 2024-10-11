// private
mod driver;
mod gate;

// usage
use crate::SerialSettings;
use crate::Error;

// public interface
pub use driver::Connector;
pub async fn get(serial_settings: &SerialSettings) -> Result<Connector, Error> {
    gate::get(serial_settings).await
}

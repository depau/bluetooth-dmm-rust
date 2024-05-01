use std::error::Error;
use std::time::Duration;

use async_recursion::async_recursion;
use async_std::future;
use btleplug::api::{Central, Peripheral, ScanFilter};
use btleplug::platform::Adapter;
use futures::stream::StreamExt;
use uuid::Uuid;

use crate::DmmError;
use crate::parser::Measurement;

const DMM_NAME: &str = "Bluetooth DMM";
const DMM_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x0000fff4_0000_1000_8000_00805f9b34fb);

#[derive(Debug)]
pub struct DmmDevice<P: Peripheral> {
    device: P,
}

impl<P: Peripheral> DmmDevice<P> {
    pub fn new(device: P) -> DmmDevice<P> {
        DmmDevice { device }
    }

    pub fn device(&self) -> &P {
        &self.device
    }

    pub async fn connect(&self) -> Result<(), Box<dyn Error>> {
        self.device.connect().await?;
        self.device.discover_services().await?;

        let chars = self.device.characteristics();
        let char = chars
            .iter()
            .find(|c| c.uuid == DMM_CHARACTERISTIC_UUID)
            .expect("Unable to find the DMM characteristic");
        self.device.subscribe(char).await?;

        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        self.device.disconnect().await?;
        Ok(())
    }

    #[async_recursion]
    pub async fn next_event(&self, timeout: Duration) -> Result<Measurement, Box<dyn Error>> {
        let mut notifications = self.device.notifications().await?;
        let data = future::timeout(timeout, async move { notifications.next().await })
            .await?
            .ok_or(DmmError::DeviceDisconnected)?;

        if data.uuid != DMM_CHARACTERISTIC_UUID {
            return self.next_event(timeout).await;
        }

        Ok(Measurement::from_bytes(data.value.as_slice().try_into()?)?)
    }
}

pub async fn scan_for_dmm(
    adapter: Adapter,
) -> Result<DmmDevice<btleplug::platform::Peripheral>, Box<dyn Error>> {
    let filter = ScanFilter {
        services: vec![DMM_CHARACTERISTIC_UUID],
    };

    adapter.start_scan(filter).await?;

    for p in adapter.peripherals().await? {
        let name = p
            .properties()
            .await?
            .unwrap()
            .local_name
            .unwrap_or_default();
        if name == DMM_NAME {
            adapter.stop_scan().await?;
            return Ok(DmmDevice::new(p));
        }
    }

    Err(DmmError::DeviceNotFound.into())
}

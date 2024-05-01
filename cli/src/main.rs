use std::error::Error;
use std::time::Duration;

use btleplug::api::{Manager, Peripheral};

use btdmm_comm::DisplayValue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Scanning for devices...");

    let manager = btleplug::platform::Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
        return Ok(());
    }

    if adapter_list.len() > 1 {
        eprintln!("Multiple Bluetooth adapters found, using the first one");
    }

    let adapter = adapter_list.into_iter().next().unwrap();

    let dmm = btdmm_comm::scan_for_dmm(adapter).await?;
    dmm.connect().await?;

    println!(
        "Connected to device: {:?} ({})",
        dmm.device()
            .properties()
            .await?
            .unwrap()
            .local_name
            .unwrap_or("Unknown".to_string()),
        dmm.device().address()
    );

    println!();

    loop {
        let measurement = dmm.next_event(Duration::from_secs(5)).await?;

        match measurement.displayed_value {
            DisplayValue::Text(text) => print!("{} ", text),
            DisplayValue::Number(value) => print!("{} ", value),
        }
        if let Some(unit) = measurement.value_unit {
            print!("{} ", unit);
        }
        print!("   ");
        print!(
            "{:?}",
            measurement
                .displayed_icons
                .iter()
                .map(|i| format!("{:?}", i))
                .collect::<Vec<String>>()
        );
        println!();
    }
}

use std::env;
use tapo::{responses::ChildDeviceHubResult, ApiClient, HubHandler, PlugEnergyMonitoringHandler};
pub mod config;
pub async fn initialize_hub(
    client: &ApiClient,
    hub_ip: &str,
) -> Result<HubHandler, Box<dyn std::error::Error>> {
    let hub = client.clone().h100(hub_ip).await?;
    Ok(hub)
}

pub async fn initialize_p100(
    client: &ApiClient,
    p100_ip: &str,
) -> Result<PlugEnergyMonitoringHandler, Box<dyn std::error::Error>> {
    let p100 = client.clone().p110(p100_ip).await?;
    Ok(p100)
}

/// Ejecuta la lógica del termostato en modo verano.
///
/// # Parámetros
/// - `hub`: conexión con el hub que mide la temperatura.
/// - `p100`: enchufe inteligente.
/// - `min_temp`: temperatura mínima aceptada.
/// - `max_temp`: temperatura máxima aceptada.
///
/// # Errores
/// Devuelve `Err` si falla al leer dispositivos o encender/apagar el enchufe.
pub async fn thermostat(
    hub: &HubHandler,
    p100: &PlugEnergyMonitoringHandler,
    min_temp: f32,
    max_temp: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let hub_children = hub.get_child_device_list().await?;
    let thermostat_enabled: bool = env::var("THERMOSTAT_ENABLED")?.parse()?;

    if !thermostat_enabled {
        println!("Thermostat is disabled. Turning P100 off.");
        p100.off().await?;
        return Ok(());
    }

    for child in hub_children {
        if let ChildDeviceHubResult::T310(device) | ChildDeviceHubResult::T315(device) = child {
            let current_temperature = device.current_temperature;

            if current_temperature > min_temp {
                println!("Temperature below minimum. Turning P100 on.");
                p100.on().await?;
            } else if current_temperature < max_temp {
                println!("Temperature above maximum. Turning P100 off.");
                p100.off().await?;
            } else {
                println!("Temperature within range. No action taken.");
            }
            return Ok(()); // Only process the first T310/T315 found
        }
    }

    println!("No T310/T315 devices found.");
    Ok(())
}

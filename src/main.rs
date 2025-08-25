use hamrothermostat::{
    config::{config, init_config, reload_config},
    initialize_hub, initialize_p100, thermostat,
};

use tapo::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration

    let _ = init_config();
    let client = ApiClient::new(&config().tapo_user, &config().tapo_password);

    // Initialize devices
    let hub = initialize_hub(&client, &config().hub_ip.to_string()).await?;
    let thermo_1 = initialize_p100(&client, &config().thermo_1_ip.to_string()).await?;
    let thermo_2 = initialize_p100(&client, &config().thermo_2_ip.to_string()).await?;

    // Run thermostat logic periodically
    loop {
        if let Err(e) = reload_config() {
            eprintln!("⚠️ no se pudo recargar .env: {e} (sigo con la config anterior)");
        }

        thermostat(&hub, &thermo_1, config().min_temp, config().max_temp).await?;
        thermostat(&hub, &thermo_2, config().min_temp, config().max_temp).await?;
        tokio::time::sleep(config().duration).await; // Run based on duration from env variable
    }
}

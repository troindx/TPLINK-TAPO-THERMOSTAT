use std::time::Duration;

use tapo::ApiClient;

// Ajusta el nombre del crate si no es "thermostat"
use hamrothermostat::{
    config::{config, init_config},
    initialize_hub, initialize_p100, thermostat,
};

/// Test de integración end-to-end sobre lib.rs:
/// - Carga configuración real (.env)
/// - Conecta al hub y lista hijos
/// - Inicializa dos P100
/// - Ciclo ON/OFF sobre un P100
/// - Ejecuta la lógica del termostato con THERMOSTAT_ENABLED=false y luego =true
#[tokio::test]
async fn lib_integration_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Cargar config real del .env
    let _ = init_config();
    let cfg = config();

    // 2) Cliente TAPO y handlers reales (tal y como defines en lib.rs)
    let client = ApiClient::new(&cfg.tapo_user, &cfg.tapo_password);

    let hub_ip = cfg.hub_ip.to_string();
    let thermo1_ip = cfg.thermo_1_ip.to_string();
    let thermo2_ip = cfg.thermo_2_ip.to_string();

    let hub = initialize_hub(&client, &hub_ip).await?;
    let p100_1 = initialize_p100(&client, &thermo1_ip).await?;
    let p100_2 = initialize_p100(&client, &thermo2_ip).await?;

    // 3) Validar que el hub devuelve algún hijo (T310/T315, etc.)
    let children = hub.get_child_device_list().await?;
    assert!(
        !children.is_empty(),
        "El hub no devolvió dispositivos hijos"
    );

    // 4) Ciclo ON/OFF sobre el primer enchufe (estado real)
    p100_1.on().await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    p100_1.off().await?;

    // 5) Rama: THERMOSTAT_ENABLED = false  → debe apagar y salir OK
    std::env::set_var("THERMOSTAT_ENABLED", "false");
    thermostat(&hub, &p100_1, cfg.min_temp, cfg.max_temp).await?;

    // 6) Rama: THERMOSTAT_ENABLED = true   → ejecuta lógica (ON/OFF según temperatura)
    std::env::set_var("THERMOSTAT_ENABLED", "true");
    thermostat(&hub, &p100_2, cfg.min_temp, cfg.max_temp).await?;

    Ok(())
}

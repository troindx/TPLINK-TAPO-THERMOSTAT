use hamrothermostat::config::Config;

#[test]
fn test_load_env_ok() {
    let cfg = Config::load().expect("No pudo cargar el .env existente");

    // Validaciones básicas de formato/contenido
    assert!(!cfg.tapo_user.is_empty());
    assert!(!cfg.tapo_password.is_empty());

    // Las IPs deben parsearse correctamente a algo válido
    assert!(cfg.hub_ip.is_ipv4());
    assert!(cfg.thermo_1_ip.is_ipv4());
    assert!(cfg.thermo_2_ip.is_ipv4());

    // Las temperaturas deben estar en un rango razonable
    assert!(cfg.min_temp > 0.0 && cfg.min_temp < 100.0);
    assert!(cfg.max_temp > cfg.min_temp && cfg.max_temp < 100.0);

    // La duración debe ser positiva
    assert!(cfg.duration.as_secs() > 0);

    // El booleano simplemente debe existir
    assert!(cfg.thermostat_enabled == true || cfg.thermostat_enabled == false);
}

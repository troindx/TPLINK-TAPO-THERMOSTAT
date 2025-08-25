use std::sync::{OnceLock, RwLock};
use std::{env, net::IpAddr, str::FromStr, time::Duration};

static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Config {
    pub tapo_user: String,
    pub tapo_password: String,
    pub hub_ip: IpAddr,
    pub thermo_1_ip: IpAddr,
    pub thermo_2_ip: IpAddr,
    pub min_temp: f32,
    pub max_temp: f32,
    pub duration: Duration,
    pub thermostat_enabled: bool,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        dotenv::dotenv().ok();

        Ok(Self {
            tapo_user: get_str("TAPO_USER")?,
            tapo_password: get_str("TAPO_PASSWORD")?,
            hub_ip: get_ip("HUB_IP")?,
            thermo_1_ip: get_ip("THERMO_1_IP")?,
            thermo_2_ip: get_ip("THERMO_2_IP")?,
            min_temp: get_parse::<f32>("MIN_TEMP")?,
            max_temp: get_parse::<f32>("MAX_TEMP")?,
            duration: Duration::from_secs(get_parse::<u64>("DURATION")?),
            thermostat_enabled: get_bool("THERMOSTAT_ENABLED")?,
        })
    }
}

fn get_str(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("Falta la variable {key}"))
}

fn get_parse<T>(key: &str) -> Result<T, String>
where
    T: FromStr,
    T::Err: ToString,
{
    let raw = get_str(key)?;
    raw.parse::<T>()
        .map_err(|e| format!("Error al parsear {key}: {}", e.to_string()))
}

fn get_ip(key: &str) -> Result<IpAddr, String> {
    get_parse::<IpAddr>(key)
}

fn get_bool(key: &str) -> Result<bool, String> {
    let raw = get_str(key)?;
    match raw.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Ok(true),
        "false" | "0" | "no" | "n" => Ok(false),
        _ => Err(format!("Valor inválido para {key}: {raw}")),
    }
}
/// Inicializa el struct Config desde dotenv y lo guarda en memoria.
/// Importante:: hay que llamar a esto primero antes de usarlo
pub fn init_config() -> &'static RwLock<Config> {
    CONFIG.get_or_init(|| RwLock::new(Config::load().expect("Error cargando configuración")))
}

/// Devuelve un guard con el objeto config (del fichero .env) de solo lectura.
pub fn config() -> std::sync::RwLockReadGuard<'static, Config> {
    CONFIG
        .get()
        .expect("CONFIG no inicializado; llama primero a init_config()")
        .read()
        .expect("RwLock poisoned")
}

/// Recarga la configuración de nuevo desde dotenv.
/// Actualiza también la referencia global CONFIG con los nuevos parámetros
/// de esta manera podemos cambiar los parámetros modificando el fichero .env sin reiniciar el servicio
pub fn reload_config() -> Result<(), String> {
    let new_cfg = Config::load()?;
    let rwlock = CONFIG
        .get()
        .ok_or("CONFIG no inicializado. Llama primero a init_config()")?;
    *rwlock.write().map_err(|_| "RwLock poisoned")? = new_cfg;
    Ok(())
}

# Rust Thermostat Controller for TP-Link Tapo Devices

This project is a Rust-based thermostat controller for TP-Link Tapo devices (H100 hub, T310/T315 sensors, and P100/P110 smart plugs).  
It allows you to **control heating (winter mode)** or **ventilation/AC (summer mode)** automatically based on temperature readings from the hub.

---

## Features
- Load configuration from a `.env` file.
- Global `Config` singleton accessible from anywhere in the project.
- Supports hot-reloading of `.env` values at runtime.
- Async logic using `tokio`.
- Integration tests that work with **real devices** (no mocks).


---

## Requirements
- Rust stable (>=1.70)
- `tokio` runtime
- Tapo hub (H100) + sensors (T310/T315)
- Tapo smart plugs (P100 or P110)

---

## Configuration

Create a `.env` file at the project root:

```env
TAPO_USER=your_username
TAPO_PASSWORD=your_password
HUB_IP=192.168.1.100
THERMO_1_IP=192.168.1.102
THERMO_2_IP=192.168.1.101
MIN_TEMP=30.0
MAX_TEMP=31.0
DURATION=300
THERMOSTAT_ENABLED=true
```

### Explanation
- **TAPO_USER / TAPO_PASSWORD** → your Tapo credentials.
- **HUB_IP** → IP address of the Tapo hub (H100).
- **THERMO_1_IP / THERMO_2_IP** → IP addresses of your P100/P110 plugs.
- **MIN_TEMP** → temperature threshold below which the device should turn off.
- **MAX_TEMP** → temperature threshold above which the device should turn on.
- **DURATION** → loop interval in seconds (e.g., `300` = 5 minutes).
- **THERMOSTAT_ENABLED** → enable/disable the thermostat logic (`true`/`false`).

---

## How It Works

- The hub (`HubHandler`) reads the current temperature from T310/T315 sensors.
- The thermostat logic decides:
  - **Summer mode (ventilation/AC):**
    - If temperature > `MAX_TEMP` → turn **on** plugs (ventilators/AC).
    - If temperature < `MIN_TEMP` → turn **off** plugs.
    - Otherwise → do nothing.
  - **Winter mode (heating)** can be swapped by reversing the conditions.
- The loop runs every `DURATION` seconds.
- The configuration (`Config`) is reloaded at each iteration → sysadmins can edit `.env` and changes are applied live.

---

## Usage

### 1. Build
```bash
cargo build --release
```

### 2. Run
```bash
cargo run
```

The program will:
1. Load the `.env`.
2. Connect to the hub and plugs.
3. Start an infinite loop, checking sensors and controlling plugs.

---

## Integration Tests

Integration tests require **real devices** and the `.env` file set correctly.  

Run all tests:
```bash
cargo test -- --test-threads=1
```

Example integration test flow:
- Loads `.env` configuration.
- Connects to hub and lists children.
- Initializes smart plugs.
- Toggles plugs ON/OFF.
- Executes thermostat logic.

> ⚠️ **Warning:** tests will physically switch your plugs on/off.

---

## Example Loop

```rust
loop {
    // Reload config from .env
    reload_config().ok();

    {
        let cfg = config();
        thermostat(&hub, &plug, cfg.min_temp, cfg.max_temp)
            .await
            .expect("Thermostat logic failed");
    }

    tokio::time::sleep(config().duration).await;
}
```

---

## Future Improvements
- Add `THERMOSTAT_MODE=summer|winter` flag to `.env`.
- Support flexible duration syntax (`5m`, `300s`).
- More robust error handling and retries.
- Optional web dashboard to monitor state.
- Switch easily between **winter** and **summer** logic.
---

## License
MIT  

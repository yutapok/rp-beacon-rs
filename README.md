# rp-beacon-rs
## About
- BLE beacon on Raspberry pi with Rust.

## Dependencies
- `rustc/cargo`
  - blueR (bluez binding for Rust)
    - https://github.com/bluez/bluer
- Raspberry pi 4

## Getting Started
### Setup
- install
```
$ sudo apt update && upgrade
$ sudo apt install libdbus-1-dev pkg-config
```

- rust install
  - https://www.rust-lang.org/ja/tools/install


### Deliver advertisement packet
```
$ cargo run

Advertising on Bluetooth adapter hci0 with address xxxxxxx 
Advertisement { advertisement_type: Peripheral, service_uuids: {xxxxx}, manufacturer_data: {}, solicit_uuids: {}, service_data: {}, advertisting_data: {}, discoverable: Some(true), discoverable_timeout: None, system_includes: {}, local_name: Some(xxxxx), appearance: None, duration: None, timeout: None, secondary_channel: None, min_interval: None, max_interval: None, tx_power: None, _non_exhaustive: () }
Press enter to quit

```


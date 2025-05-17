# PixelWeather Configuration Generator for Home Assistant

A tool for easily generating configuration files containing entity configurations all of your PixelWeather nodes. This allows you to integrate your PixelWeather nodes with [Home Assistant](https://www.home-assistant.io/), a FOSS home automation system. All sensor data is pulled from the same database as the one used by the [PWMP server](https://github.com/PixelWeatherProject/pwmp-server).

This tool requires a pre-configured database (see [this](https://github.com/PixelWeatherProject/pwmp-server)). Since the PWMP server only supports PostgreSQL, this tool also has the same restriction.

## CLI
```
Home Assistant configuration generator for PixelWeather.

Usage: pw-hassgen [OPTIONS] --username <USERNAME> --password <PASSWORD> --database <DATABASE> <HOST> <COMMAND>

Commands:
  generate  Generate YAML configuration
  help      Print this message or the help of the given subcommand(s)

Arguments:
  <HOST>  Database Host

Options:
      --port <PORT>          Database Port [default: 5432]
  -u, --username <USERNAME>  Database username
  -p, --password <PASSWORD>  Database password
  -d, --database <DATABASE>  Database name
  -h, --help                 Print help
  -V, --version              Print version
```
The `generate` command provides additional options for more customization.

## Example output
```yml
- name: Node 1 Temperature
  query: SELECT CAST(temperature AS DECIMAL(4, 2)) FROM measurements WHERE node = 1 ORDER BY "when" DESC LIMIT 1;
  column: temperature
  db_url: postgresql://user:pass@192.168.0.111:5432/pixelweather
  unit_of_measurement: °C
  device_class: temperature
  state_class: MEASUREMENT
  icon: mdi:thermometer
- name: Node 1 Humidity
  query: SELECT humidity FROM measurements WHERE node = 1 ORDER BY "when" DESC LIMIT 1;
  column: humidity
  db_url: postgresql://user:pass@192.168.0.111:5432/pixelweather
  unit_of_measurement: '%'
  device_class: humidity
  state_class: MEASUREMENT
  icon: mdi:water-percent
- name: Node 1 Battery
  query: SELECT CAST(battery AS DECIMAL(3, 2)) FROM statistics JOIN measurements ON measurements.id = statistics.measurement WHERE measurements.node = 1 ORDER BY "when" DESC LIMIT 1;
  column: battery
  db_url: postgresql://user:pass@192.168.0.111:5432/pixelweather
  unit_of_measurement: V
  device_class: voltage
  state_class: MEASUREMENT
  icon: mdi:battery
- name: Node 1 WiFi ESSID
  query: SELECT wifi_ssid FROM statistics JOIN measurements ON measurements.id = statistics.measurement WHERE measurements.node = 1 ORDER BY "when" DESC LIMIT 1;
  column: wifi_ssid
  db_url: postgresql://user:pass@192.168.0.111:5432/pixelweather
  icon: mdi:wifi
- name: Node 1 WiFi Signal Strength
  query: SELECT wifi_rssi FROM statistics JOIN measurements ON measurements.id = statistics.measurement WHERE measurements.node = 1 ORDER BY "when" DESC LIMIT 1;
  column: wifi_rssi
  db_url: postgresql://user:pass@192.168.0.111:5432/pixelweather
  unit_of_measurement: dBm
  device_class: signal_strength
  state_class: MEASUREMENT
  icon: mdi:signal-cellular-2
```

## Caveats
- **All database connection info (incl. username and password) are stored in plain-text! Unfortunately, as of now, Home Assistant does NOT provide an alternative solution.**
- Currently, it's not possible to read **nor** change node settings.
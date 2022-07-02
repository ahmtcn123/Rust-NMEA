# Rust-NMEA

[![Crates.io Version](https://img.shields.io/crates/v/rust_nmea?logo=rust)](https://crates.io/crates/rust_nmea)
[![Documentation](https://docs.rs/rust_nmea/badge.svg)](https://docs.rs/rust_nmea)

NMEA (0183) parser and information library for Rust.

## Example

```rust
    use rust_nmea::{parser, types::{CommandTypes, Time, Cordinate, GGAStatus}, commands::gga::GGA};
    let line = "$GPGGA,161009.00,1122.20418,N,02339.35234,E,1,08,1.09,11.5,M,11.3,M,,*62";
    let parsed = parser::Parser::parse_line(line);
    assert_eq!(parsed, Ok(
       CommandTypes::GGA(GGA {
           time: Time {
               hour: 16,
               minute: 10,
               second: 9,
               decimal_seconds: 0,
           },
           lat: Cordinate {
               degree: 11,
               minute: 22.20418,
           },
           northing_indicator: 'N',
           lon: Cordinate {
               degree: 2,
               minute: 339.42234,
           },
           easting_indicator: 'E',
           status: GGAStatus::S2d3D,
           number_of_satellites: 8,
           horizontal_dilution_of_position: 1.09,
           altitude: 11.5,
           altitude_unit: "M".to_string(),
           geoid_separation: 11.3,
           geoid_separation_unit: "M".to_string(),
           differential_age_of_position: 0,
           differential_reference_station_id: 0,
      })
    ));
```

| Supported Packages                          | Tested | Implemented |
| ------------------------------------------- | ------ | ----------- |
| [DTM] (Datum Reference)                     | ❌     | ❌          |
| [GBS] (Satellite Fault Detection)           | ❌     | ❌          |
| [GGA] (Global Positioning System Fix Data)  | ✅     | ✅          |
| [GLL] (Geographic Position - Lat / Long)    | ✅     | ✅          |
| [GNS] (GNSS Fix Data)                       | ❌     | ❌          |
| [GRS] (GNSS Range Residuals)                | ❌     | ❌          |
| [GSA] (GNSS DOP and Active Satellites)      | ✅     | ✅          |
| [GST] (GNSS Pseudorange Error Statistics)   | ❌     | ❌          |
| [GSV] (GNSS Satellites in View)             | ✅     | ✅          |
| [RLM] (Return Link Message)                 | ❌     | ❌          |
| [RMC] (Recommended Min Specific GNSS Data)  | ✅     | ✅          |
| [THS] (Heading of Vehicle)                  | ❌     | ❌          |
| [TXT] (Text Transmission)                   | ❌     | ❌          |
| [VLW] (Dual Ground / Water Distance)        | ❌     | ❌          |
| [VTG] (Course Over Ground and Ground Speed) | ✅     | ✅          |
| [ZDA] (Time & Date)                         | ✅     | ✅          |

Important: This libary does not provide SerialPort

## License

Rust-NMEA is licensed under the [GPL-2.0 license](./LICENSE)

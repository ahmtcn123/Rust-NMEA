#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]
#![doc(html_root_url = "https://docs.rs/rust_nmea/1.2.6")]
//!# Rust NMEA
//!NMEA (0183) parser and information library for Rust.
//!
//!## Usage
//!
//! ```
//! use rust_nmea::{parser, types::{CommandTypes, Time, CardinalDirection, Cordinate, GGAStatus}, commands::gga::GGA};
//! let line = "$GPGGA,161009.00,1122.20418,N,02339.35234,E,1,08,1.09,11.5,M,11.3,M,,*62";
//! let parsed = parser::Parser::parse_line(line);
//! assert_eq!(parsed, Ok(
//!    CommandTypes::GGA(GGA {
//!        time: Time {
//!            hour: 16,
//!            minute: 10,
//!            second: 9,
//!            decimal_seconds: 0,
//!        },
//!        lat: Cordinate {
//!            degree: 112,
//!            minute: 2.20418,
//!        },
//!        northing_indicator: CardinalDirection::North,
//!        lon: Cordinate {
//!            degree: 23,
//!            minute: 39.35234,
//!        },
//!        easting_indicator: CardinalDirection::East,
//!        status: GGAStatus::S2d3D,
//!        number_of_satellites: 8,
//!        horizontal_dilution_of_position: 1.09,
//!        altitude: 11.5,
//!        altitude_unit: "M".to_string(),
//!        geoid_separation: 11.3,
//!        geoid_separation_unit: "M".to_string(),
//!        differential_age_of_position: 0.0,
//!        differential_reference_station_id: 0,
//!   })
//! ));
//! ```
//! You can find more examples [here](https://github.com/ahmtcn123/Rust-NMEA/master/examples)

/// NMEA commands
pub mod commands;
/// Parse
pub mod parser;
/// Types
pub mod types;

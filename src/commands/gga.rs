use std::vec;

use crate::types::{CardinalDirection, Command, Cordinate, Error, GGAStatus, Time};

/// GGA (Global Positioning System Fix Data)
#[derive(Debug, Clone, PartialEq)]
pub struct GGA {
    /// UTC Time in hhmmss.ss format
    pub time: Time,
    /// Latitude in ddmm.mmmmm format
    pub lat: Cordinate,
    /// N=North/S=South indicator
    pub northing_indicator: CardinalDirection,
    /// Longitude in dddmm.mmmmm format
    pub lon: Cordinate,
    /// E=East/W=West indicator
    pub easting_indicator: CardinalDirection,
    /// GPS Quality Indicator
    pub status: GGAStatus,
    /// Number of satellites in view
    pub number_of_satellites: u8,
    /// Horizontal dilution of precision
    pub horizontal_dilution_of_position: f32,
    /// Antenna altitude above mean sea level
    pub altitude: f64,
    /// Altitude units: M (meters, fixed field)
    pub altitude_unit: String,
    /// Geoid separation: difference between ellipsoid and mean sea level
    pub geoid_separation: f64,
    /// Geoid separation units: M (meters, fixed field)
    pub geoid_separation_unit: String,
    /// Age of differential corrections (null when DGPS is not used)
    pub differential_age_of_position: usize,
    /// Differential reference station ID (null when DGPS is not used)
    pub differential_reference_station_id: usize,
}

impl Default for GGA {
    fn default() -> Self {
        Self {
            time: Default::default(),
            lat: Default::default(),
            lon: Default::default(),
            status: GGAStatus::Invalid,
            number_of_satellites: Default::default(),
            horizontal_dilution_of_position: Default::default(),
            altitude: Default::default(),
            altitude_unit: Default::default(),
            geoid_separation: Default::default(),
            geoid_separation_unit: Default::default(),
            differential_age_of_position: Default::default(),
            differential_reference_station_id: Default::default(),
            northing_indicator: CardinalDirection::North,
            easting_indicator: CardinalDirection::East,
        }
    }
}

impl Command<GGA> for GGA {
    fn parse_command(&self, command: Vec<String>) -> Result<GGA, Error> {
        if command.len() != 14 && command.len() != 13 {
            Err(Error::ParseError(format!(
                "Invalid command length for GGA: {}",
                command.len()
            )))
        } else {
            let time_split: Vec<&str> = if command[0].contains(".") {
                command[0].split(".").collect()
            } else {
                vec![&command[0], "0"]
            };

            let hour = time_split[0][..2].parse()?;
            let minute = time_split[0][2..4].parse()?;
            let second = time_split[0][4..6].parse()?;
            let decimal_seconds = time_split[1].parse()?;
            let time = Time {
                hour,
                minute,
                second,
                decimal_seconds,
            };

            let latitude_degree: usize = command[1][..3].parse()?;
            let latitude_minute = command[1][3..].parse()?;
            let northing_indicator = match command[2].chars().next() {
                Some(e) => e,
                None => return Err(Error::ParseError("Invalid nothing indicator".to_string())),
            };

            let northing_indicator = match CardinalDirection::from_char(northing_indicator) {
                Some(e) => e,
                None => return Err(Error::ParseError("Invalid northing indicator".to_string())),
            };

            let longitude_degree: usize = command[3][..3].parse()?;
            let longitude_minute = command[3][3..].parse()?;
            let easting_indicator = match command[4].chars().next() {
                Some(e) => e,
                None => return Err(Error::ParseError("Invalid easting indicator".to_string())),
            };

            let easting_indicator = match CardinalDirection::from_char(easting_indicator) {
                Some(e) => e,
                None => return Err(Error::ParseError("Invalid easting indicator".to_string())),
            };

            let lat = Cordinate {
                degree: latitude_degree,
                minute: latitude_minute,
            };

            let lon = Cordinate {
                degree: longitude_degree,
                minute: longitude_minute,
            };

            let status: GGAStatus = match command[5].parse::<u8>()? {
                0 => Ok(GGAStatus::Invalid),
                1 => Ok(GGAStatus::S2d3D),
                2 => Ok(GGAStatus::Dgnss),
                3 => Ok(GGAStatus::FixedRtk),
                4 => Ok(GGAStatus::FloatRtk),
                5 => Ok(GGAStatus::DeadReckoning),
                _ => Err(Error::ParseError(format!(
                    "Invalid status for GGA: {}",
                    command[5]
                ))),
            }?;
            let number_of_satellites: u8 = command[6].parse()?;
            let horizontal_dilution_of_position: f32 = command[7].parse()?;
            let altitude = command[8].parse()?;
            let altitude_unit = command[9].to_string();
            let geoid_separation = command[10].parse()?;
            let geoid_separation_unit = command[11].to_string();
            let differential_age_of_position = if command[12] == "" {
                0_usize
            } else {
                command[12].parse()?
            };
            let differential_reference_station_id = if command.len() == 14 {
                0
            } else if command[13] == "" {
                0
            } else {
                command[13].parse()?
            };

            Ok(GGA {
                time,
                lat,
                northing_indicator,
                lon,
                easting_indicator,
                status,
                number_of_satellites,
                horizontal_dilution_of_position,
                altitude,
                altitude_unit,
                geoid_separation,
                geoid_separation_unit,
                differential_age_of_position,
                differential_reference_station_id,
            })
        }
    }
}

use crate::types::{Command, Cordinate, Error, ModeIndicator, Status, Time};

/// GLL ( Geographic Position - Latitude/Longitude )
#[derive(Debug, Clone, PartialEq)]
pub struct GLL {
    /// Latitude in ddmm.mmmm format
    pub lat: Cordinate,
    /// N=North/S=South indicator
    pub northing_indicator: char,
    /// Longitude in dddmm.mmmm format
    pub lon: Cordinate,
    /// E=East/W=West indicator
    pub easting_indicator: char,
    /// UTC Time in hhmmss.sss format
    pub time: Time,
    /// Status
    pub status: Status,
    /// Mode Indicator
    pub mode_indicator: ModeIndicator,
}

impl Default for GLL {
    fn default() -> Self {
        Self {
            lat: Default::default(),
            lon: Default::default(),
            time: Default::default(),
            status: Status::Invalid,
            mode_indicator: ModeIndicator::None,
            northing_indicator: Default::default(),
            easting_indicator: Default::default(),
        }
    }
}

impl Command<GLL> for GLL {
    fn parse_command(&self, command: Vec<String>) -> Result<GLL, crate::types::Error> {
        let latitude_degree: usize = command[0][..2].parse()?;
        let latitude_minute: f32 = command[0][2..].parse()?;
        let northing_indicator = match command[1].chars().next() {
            Some(e) => e,
            None => return Err(Error("Invalid northing indicator".to_string())),
        };

        let longitude_degree: usize = command[2][..2].parse()?;
        let longitude_minute: f32 = command[2][2..].parse()?;
        let easting_indicator = match command[3].chars().next() {
            Some(e) => e,
            None => return Err(Error("Invalid easting indicator".to_string())),
        };

        let lat = Cordinate {
            degree: latitude_degree,
            minute: latitude_minute,
        };

        let lon = Cordinate {
            degree: longitude_degree,
            minute: longitude_minute,
        };

        let time_split: Vec<&str> = command[4].split(".").collect();

        let hour = time_split[0][..2].parse::<u8>()?;
        let minute = time_split[0][2..4].parse::<u8>()?;
        let second = time_split[0][4..6].parse::<u8>()?;
        let decimal_seconds = time_split[1].parse::<u8>()?;
        let time = Time {
            hour,
            minute,
            second,
            decimal_seconds,
        };
        let status = match Status::from_str(&command[5]) {
            Ok(e) => e,
            Err(_) => {
                return Err(Error("Invalid status".to_string()));
            }
        };
        let mode_indicator = match ModeIndicator::from_str(&command[6]) {
            Ok(e) => e,
            Err(_) => return Err(Error("Invalid mode indicator".to_string())),
        };

        Ok(GLL {
            lat,
            northing_indicator,
            lon,
            easting_indicator,
            time,
            status,
            mode_indicator,
        })
    }
}

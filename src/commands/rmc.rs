use crate::types::{CardinalDirection, Command, Cordinate, Date, Error, Status, Time};

/// RMC ( Recommended Minimum Specific GPS Data )
#[derive(Debug, Clone, PartialEq)]
pub struct RMC {
    /// UTC Time in hhmmss.sss format
    pub time: Time,
    /// Status
    pub status: Status,
    /// Latitude in ddmm.mmmm format
    pub lat: Cordinate,
    /// N=North/S=South indicator
    pub northing_indicator: CardinalDirection,
    /// Longitude in dddmm.mmmm format
    pub lon: Cordinate,
    /// E=East/W=West indicator
    pub easting_indicator: CardinalDirection,
    /// Speed over ground in knots
    pub speed_over_ground: f32,
    /// Course over ground in degrees
    pub course_over_ground: Option<f32>,
    /// UTC Date
    pub date: Date,
    /// Magnetic variation degrees (Easterly varient is positive)
    pub magnetic_variation: Option<f64>,
    /// Magnetic variation E=East/W=West indicator, it could be empty
    pub magnetic_variation_indicator: Option<CardinalDirection>,
}

impl Default for RMC {
    fn default() -> Self {
        Self {
            time: Default::default(),
            status: Status::Invalid,
            lat: Default::default(),
            northing_indicator: CardinalDirection::North,
            lon: Default::default(),
            easting_indicator: CardinalDirection::East,
            speed_over_ground: Default::default(),
            course_over_ground: Default::default(),
            date: Default::default(),
            magnetic_variation: Default::default(),
            magnetic_variation_indicator: None,
        }
    }
}

impl Command<RMC> for RMC {
    fn parse_command(&self, command: Vec<String>) -> Result<RMC, crate::types::Error> {
        let time_split: Vec<&str> = if command[0].contains(".") {
            command[0].split(".").collect()
        } else {
            vec![&command[0], "0"]
        };

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

        let status = match Status::from_str(&command[1]) {
            Ok(e) => e,
            Err(_) => return Err(Error::ParseError("Invalid status".to_string())),
        };

        let latitude_degree = command[2][..3].parse()?;
        let latitude_minute = command[2][3..].parse()?;
        let northing_indicator = match command[3].chars().next() {
            Some(e) => e,
            None => return Err(Error::ParseError("Invalid northing indicator".to_string())),
        };

        let northing_indicator = match CardinalDirection::from_char(northing_indicator) {
            Some(e) => e,
            None => return Err(Error::ParseError("Invalid northing indicator".to_string())),
        };

        let longitude_degree = command[4][..3].parse()?;
        let longitude_minute = command[4][3..].parse()?;
        let easting_indicator = match command[5].chars().next() {
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

        let speed_over_ground = command[6].parse()?;
        let course_over_ground = command[7].parse::<f32>().ok();

        let date_split = &command[8];
        let day = date_split[..2].parse::<u8>()?;
        let month = date_split[2..4].parse::<u8>()?;
        let year = date_split[4..].parse::<usize>()?;

        let date = Date { day, month, year };

        let magnetic_variation = match command[9].parse::<f64>() {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        let magnetic_variation_indicator = match command[10].chars().next() {
            Some(e) => {
                if e == 'E' {
                    'E'
                } else if e == 'W' {
                    'W'
                } else if e == ' ' {
                    ' '
                } else if e == '\0' {
                    ' '
                } else {
                    return Err(Error::ParseError(
                        "Invalid magnetic variation indicator".to_string(),
                    ));
                }
            }
            None => ' ',
        };

        let magnetic_variation_indicator =
            match CardinalDirection::from_char(magnetic_variation_indicator) {
                Some(e) => Some(e),
                None => None,
            };

        Ok(RMC {
            time,
            status,
            lat,
            northing_indicator,
            lon,
            easting_indicator,
            speed_over_ground,
            course_over_ground,
            date,
            magnetic_variation,
            magnetic_variation_indicator,
        })
    }
}

use crate::types::{
    Command, Cordinate, Date, Error, ModeIndicator, NavigationalStatus, Status, Time,
};

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
    pub northing_indicator: char,
    /// Longitude in dddmm.mmmm format
    pub lon: Cordinate,
    /// E=East/W=West indicator
    pub easting_indicator: char,
    /// Speed over ground in knots
    pub speed_over_ground: f32,
    /// Course over ground in degrees
    pub course_over_ground: Option<f32>,
    /// UTC Date
    pub date: Date,
    /// Magnetic variation degrees (Easterly varient is positive)
    pub magnetic_variation: Option<f32>,
    /// E=East/W=West indicator
    pub magnetic_variation_indicator: char,
    /// Mode Indicator
    pub mode_indicator: Option<ModeIndicator>,
    /// Navigational status
    pub navigational_status: Option<NavigationalStatus>,
}

impl Default for RMC {
    fn default() -> Self {
        Self {
            time: Default::default(),
            status: Status::Invalid,
            lat: Default::default(),
            northing_indicator: Default::default(),
            lon: Default::default(),
            easting_indicator: Default::default(),
            speed_over_ground: Default::default(),
            course_over_ground: Default::default(),
            date: Default::default(),
            magnetic_variation: Default::default(),
            magnetic_variation_indicator: Default::default(),
            mode_indicator: None,
            navigational_status: None,
        }
    }
}

impl Command<RMC> for RMC {
    fn parse_command(&self, command: Vec<String>) -> Result<RMC, crate::types::Error> {
        println!("{:?}", command);

        let time_split: Vec<&str> = command[0].split(".").collect();

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
            Err(_) => return Err(Error("Invalid status".to_string())),
        };

        let latitude_degree: usize = command[2][..2].parse()?;
        let latitude_minute: f32 = command[2][2..].parse()?;
        let northing_indicator = match command[3].chars().next() {
            Some(e) => e,
            None => return Err(Error("Invalid northing indicator".to_string())),
        };

        let longitude_degree: usize = command[4][..2].parse()?;
        let longitude_minute: f32 = command[4][2..].parse()?;
        let easting_indicator = match command[5].chars().next() {
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

        let speed_over_ground = command[6].parse()?;
        let course_over_ground = command[7].parse::<f32>().ok();

        let date_split = &command[8];
        let day = date_split[..2].parse::<u8>()?;
        let month = date_split[2..4].parse::<u8>()?;
        let year = date_split[4..].parse::<usize>()?;

        let date = Date { day, month, year };
        let magnetic_variation_indicator = match command[9].chars().next() {
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
                    return Err(Error("Invalid magnetic variation indicator".to_string()));
                }
            }
            None => ' ',
        };

        let mode_indicator = match ModeIndicator::from_str(&command[10]) {
            Ok(mode) => Some(mode),
            Err(_) => None,
        };

        let navigational_status = match NavigationalStatus::from_str(&command[11]) {
            Ok(status) => Some(status),
            Err(_) => None,
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
            magnetic_variation: Some(0.0_f32),
            magnetic_variation_indicator,
            mode_indicator,
            navigational_status,
        })
    }
}

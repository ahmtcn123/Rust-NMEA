use crate::types::{CardinalDirection, Command, Error, ModeIndicator, Time};

/// GNS - GNSS Fix Data
#[derive(Debug, Clone, PartialEq)]
pub struct GNS {
    /// UTC Time
    pub time: Time,
    /// Latitude
    pub latitude: Option<f64>,
    /// Direction of latitude, N=north or S=south
    pub latitude_direction: Option<CardinalDirection>,
    /// Longitude
    pub longitude: Option<f64>,
    /// Direction of longitude, E=east or W=west
    pub longitude_direction: Option<CardinalDirection>,
    /// Mode indicator for GPS
    pub gps_mode_indicator: Option<ModeIndicator>,
    /// Mode indicator for GLONASS
    pub glonass_mode_indicator: Option<ModeIndicator>,
    /// Mode indicator for Galileo
    pub galileo_mode_indicator: Option<ModeIndicator>,
    /// Mode indicator for BeiDou
    pub beidou_mode_indicator: Option<ModeIndicator>,
    /// Mode indicator for QZSS
    pub qzss_mode_indicator: Option<ModeIndicator>,
    /// Number of Satellites view in use, range from 0 to 99
    pub number_of_satellites_in_use: u8,
    /// Horizontal Dilution of Precision, range from 0.5 to 99.9
    pub horizontal_dilution_of_precision: Option<f64>,
    /// Orthometric height (MSL reference) in meters.
    pub orthometric_height: Option<f64>,
    /// Geodial separation in meters. '-' means mean sea level in this case it will be presented as None
    pub geodial_separation: Option<f64>,
    /// Age of differential GPS data in seconds
    pub age_of_differential_gps_data: f64,
    /// Reference station ID, it could be null
    pub reference_station_id: Option<String>,
}

impl Default for GNS {
    fn default() -> Self {
        Self {
            time: Default::default(),
            latitude: Default::default(),
            latitude_direction: None,
            longitude: Default::default(),
            longitude_direction: None,
            gps_mode_indicator: Default::default(),
            glonass_mode_indicator: Default::default(),
            galileo_mode_indicator: Default::default(),
            beidou_mode_indicator: Default::default(),
            qzss_mode_indicator: Default::default(),
            number_of_satellites_in_use: Default::default(),
            horizontal_dilution_of_precision: Default::default(),
            orthometric_height: Default::default(),
            geodial_separation: Default::default(),
            age_of_differential_gps_data: Default::default(),
            reference_station_id: Default::default(),
        }
    }
}

impl Command<GNS> for GNS {
    fn parse_command(&self, command: Vec<String>) -> Result<GNS, Error> {
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

        let latitude = match command[1].parse() {
            Ok(latitude) => Some(latitude),
            Err(_) => None,
        };

        let latitude_direction = match command[2].parse::<char>() {
            Ok(direction) => CardinalDirection::from_char(direction),
            Err(_) => None,
        };

        let longitude = match command[3].parse() {
            Ok(longitude) => Some(longitude),
            Err(_) => None,
        };

        let longitude_direction = match command[4].parse::<char>() {
            Ok(direction) => CardinalDirection::from_char(direction),
            Err(_) => None,
        };

        let mode_indicator = command[5].chars();

        let gps_mode_indicator = match mode_indicator.clone().nth(0) {
            Some(mode) => ModeIndicator::from_char(mode),
            None => None,
        };

        let glonass_mode_indicator = match mode_indicator.clone().nth(1) {
            Some(mode) => ModeIndicator::from_char(mode),
            None => None,
        };

        let galileo_mode_indicator = match mode_indicator.clone().nth(2) {
            Some(mode) => ModeIndicator::from_char(mode),
            None => None,
        };

        let beidou_mode_indicator = match mode_indicator.clone().nth(3) {
            Some(mode) => ModeIndicator::from_char(mode),
            None => None,
        };

        let qzss_mode_indicator = match mode_indicator.clone().nth(4) {
            Some(mode) => ModeIndicator::from_char(mode),
            None => None,
        };

        let number_of_satellites_in_use = command[6].parse()?;

        let horizontal_dilution_of_precision = match command[7].parse::<f64>() {
            Ok(dilution) => Some(dilution),
            Err(_) => None,
        };

        let orthometric_height = match command[8].parse::<f64>() {
            Ok(height) => Some(height),
            Err(_) => None,
        };

        let geodial_separation = match command[9].parse::<f64>() {
            Ok(separation) => Some(separation),
            Err(_) => None,
        };

        let age_of_differential_gps_data = command[10].parse()?;
        let reference_station_id = match command[11].parse::<u16>() {
            Ok(id) => Some(id.to_string()),
            Err(_) => None,
        };

        Ok(GNS {
            time,
            latitude,
            latitude_direction,
            longitude,
            longitude_direction,
            gps_mode_indicator,
            glonass_mode_indicator,
            galileo_mode_indicator,
            beidou_mode_indicator,
            qzss_mode_indicator,
            number_of_satellites_in_use,
            horizontal_dilution_of_precision,
            orthometric_height,
            geodial_separation,
            age_of_differential_gps_data,
            reference_station_id,
        })
    }
}

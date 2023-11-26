use crate::types::{CardinalDirection, Command, Error};

/// DTM (Datum Reference)
#[derive(Debug, Clone, PartialEq)]
pub struct DTM {
    /// Local datum
    pub local_datum_code: String,
    /// Local datum sub division code
    pub local_datum_sub_division_code: String,
    /// Latitude offset, in minutes
    pub latitude_offset: f64,
    /// N=North/S=South indicator
    pub northing_indicator: CardinalDirection,
    /// Longitude offset, in minutes
    pub longitude_offset: f64,
    /// E=East/W=West indicator
    pub easting_indicator: CardinalDirection,
    /// Altitude offset, in meters
    pub altitude_offset: f64,
    /// Reference datum code
    pub reference_datum_code: String,
}

impl Default for DTM {
    fn default() -> Self {
        Self {
            local_datum_code: Default::default(),
            local_datum_sub_division_code: Default::default(),
            latitude_offset: Default::default(),
            northing_indicator: CardinalDirection::North,
            longitude_offset: Default::default(),
            easting_indicator: CardinalDirection::East,
            altitude_offset: Default::default(),
            reference_datum_code: Default::default(),
        }
    }
}

impl Command<DTM> for DTM {
    fn parse_command(&self, command: Vec<String>) -> Result<DTM, Error> {
        let local_datum_code = command[0].clone();
        let local_datum_sub_division_code = command[1].clone();
        let latitude_offset = command[2].parse::<f64>()?;
        let northing_indicator = match command[3].chars().next() {
            Some(e) => e,
            None => return Err(Error::ParseError("Invalid northing indicator".to_string())),
        };

        let northing_indicator = match CardinalDirection::from_char(northing_indicator) {
            Some(e) => e,
            None => return Err(Error::ParseError("Invalid northing indicator".to_string())),
        };

        let longitude_offset = command[4].parse::<f64>()?;
        let easting_indicator = match command[5].chars().next() {
            Some(e) => e,
            None => return Err(Error::ParseError("Invalid easting indicator".to_string())),
        };

        let easting_indicator = match CardinalDirection::from_char(easting_indicator) {
            Some(e) => e,
            None => return Err(Error::ParseError("Invalid easting indicator".to_string())),
        };

        let altitude_offset = command[6].parse::<f64>()?;
        let reference_datum_code = command[7].clone();

        Ok(DTM {
            local_datum_code,
            local_datum_sub_division_code,
            latitude_offset,
            northing_indicator,
            longitude_offset,
            easting_indicator,
            altitude_offset,
            reference_datum_code,
        })
    }
}

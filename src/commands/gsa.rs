use crate::types::{Command, Error, GSAOperationMode, NavigationMode};

/// GLL ( Geographic Position - Latitude/Longitude )
#[derive(Debug, Clone, PartialEq)]
pub struct GSA {
    /// Operation mode
    pub operation_mode: GSAOperationMode,
    /// Navigation mode
    pub navigation_mode: NavigationMode,
    /// IDs of satellites used in navigation
    pub satellites: Vec<Option<u8>>,
    /// Number of satellites used in navigation
    pub number_of_satellites: usize,
    /// PDOP ( Position Dilution of Precision )
    pub pdop: f32,
    /// HDOP ( Horizontal Dilution of Precision )
    pub hdop: f32,
    /// VDOP ( Vertical Dilution of Precision )
    pub vdop: f32,
}

impl Default for GSA {
    fn default() -> Self {
        Self {
            operation_mode: GSAOperationMode::Automatic,
            navigation_mode: NavigationMode::NoFix,
            satellites: Default::default(),
            number_of_satellites: Default::default(),
            pdop: Default::default(),
            hdop: Default::default(),
            vdop: Default::default(),
        }
    }
}

impl Command<GSA> for GSA {
    fn parse_command(&self, command: Vec<String>) -> Result<GSA, crate::types::Error> {
        let operation_mode = match GSAOperationMode::from_str(&command[0]) {
            Ok(e) => e,
            Err(_) => return Err(Error(format!("Invalid operation mode: {}", command[0]))),
        };
        let navigation_mode = match NavigationMode::from_str(&command[1]) {
            Ok(e) => e,
            Err(_) => return Err(Error(format!("Invalid navigation mode: {}", command[1]))),
        };
        let satellites: Vec<Option<u8>> = command[2..14]
            .iter()
            .map(|e| e.parse::<u8>().ok())
            .collect();
        let number_of_satellites = satellites
            .iter()
            .filter(|x| x.is_some())
            .collect::<Vec<_>>()
            .len();
        let pdop = command[14].parse().unwrap();
        let hdop = command[15].parse().unwrap();
        let vdop = command[16].parse().unwrap();
        Ok(GSA {
            operation_mode,
            navigation_mode,
            satellites,
            number_of_satellites,
            pdop,
            hdop,
            vdop,
        })
    }
}

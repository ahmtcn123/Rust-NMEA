use crate::{
    commands::gga::GGA,
    types::{CommandTypes, Error, TalkerIds},
};

/// Parser struct
#[derive(Debug, Clone)]
pub struct Parser {
    /// CommandType
    pub r#type: CommandTypes,
    /// Commands to parse
    pub commands: Vec<String>,
    /// Talker ID
    pub talker_id: TalkerIds,
    type_start_collected: bool,
    command_type_collected: bool,
}

impl Parser {
    /// Parse given line
    /// # Examples
    /// ```
    /// use rust_nmea::{parser, types::{CommandTypes, Time, CardinalDirection, Cordinate, GGAStatus}, commands::gga::GGA};
    /// let line = "$GPGGA,161009.00,1122.20418,N,02339.35234,E,1,08,1.09,11.5,M,11.3,M,,*62";
    /// let parsed = parser::Parser::parse_line(line);
    /// assert_eq!(parsed, Ok(
    ///    CommandTypes::GGA(GGA {
    ///        time: Time {
    ///            hour: 16,
    ///            minute: 10,
    ///            second: 9,
    ///            decimal_seconds: 0,
    ///        },
    ///        lat: Cordinate {
    ///            degree: 112,
    ///            minute: 2.20418,
    ///        },
    ///        northing_indicator: CardinalDirection::North,
    ///        lon: Cordinate {
    ///            degree: 23,
    ///            minute: 39.35234,
    ///        },
    ///        easting_indicator: CardinalDirection::East,
    ///        status: GGAStatus::S2d3D,
    ///        number_of_satellites: 8,
    ///        horizontal_dilution_of_position: 1.09,
    ///        altitude: 11.5,
    ///        altitude_unit: "M".to_string(),
    ///        geoid_separation: 11.3,
    ///        geoid_separation_unit: "M".to_string(),
    ///        differential_age_of_position: 0.0,
    ///        differential_reference_station_id: 0,
    ///   })
    /// ));
    pub fn parse_line(line: &str) -> Result<CommandTypes, Error> {
        let mut parser = Parser {
            r#type: CommandTypes::GGA(GGA::default()),
            talker_id: TalkerIds::GP,
            commands: Vec::new(),
            type_start_collected: false,
            command_type_collected: false,
        };

        let mut command = String::new();
        let mut commands: Vec<String> = Vec::new();
        let checksum = match line.split('*').last() {
            Some(e) => e,
            None => {
                return Err(Error::ParseError("Invalid line".to_string()));
            }
        };

        //Parse hexa decimal checksum to u8
        let checksum_u8: u8 = match u8::from_str_radix(checksum, 16) {
            Ok(e) => e,
            Err(_) => {
                return Err(Error::ParseError(format!(
                    "Invalid line, checksum is invalid \"{}\"",
                    checksum
                )));
            }
        };

        let command_clean = match line.split('$').last() {
            Some(e) => e,
            None => {
                return Err(Error::ParseError("Invalid line".to_string()));
            }
        };

        let command_clean = command_clean.split('*').collect::<Vec<_>>()[0];

        //nmea checksum calculation
        let mut checksum_calculated = 0;
        for c in command_clean.chars() {
            checksum_calculated ^= c as u8;
        }

        if checksum_calculated != checksum_u8 {
            return Err(Error::ChecksumError(checksum_u8, checksum_calculated));
        }

        for char in line.chars() {
            command += &char.to_string();
            if !parser.type_start_collected {
                if command.len() == 3 {
                    if TalkerIds::is_correct(&command) {
                        parser.type_start_collected = true;
                        parser.talker_id = TalkerIds::parse(&command);
                        command = "".to_string();
                    } else {
                        return Err(Error::ParseError(format!(
                            "Invalid command start \"{}\"",
                            command
                        )));
                    }
                }
            } else if !parser.command_type_collected {
                if command.len() == 4 && char == ',' {
                    command = command.replace(',', "");
                    match CommandTypes::from_str(&command) {
                        Ok(command_type) => {
                            parser.r#type = command_type;
                            parser.command_type_collected = true;
                            command = "".to_string();
                        }
                        Err(_) => {
                            return Err(Error::UnknownCommand(command));
                        }
                    }
                }
            } else if char == ',' {
                commands.push(command.replace(',', ""));
                command = "".to_string();
            } else if char == '*' {
                commands.push(command.replace([',', '*'], ""));
                command = "".to_string();
                break;
            }
        }
        if !command.is_empty() {
            commands.push(command);
        }

        if parser.command_type_collected && parser.type_start_collected {
            parser.r#type.parse_commands(commands)
        } else {
            Err(Error::ParseError("Invalid line".to_string()))
        }
    }
}

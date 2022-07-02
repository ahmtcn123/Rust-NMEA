use crate::types::{Command, Date, Error, Time};

/// ZDA (Time and Date)
#[derive(Debug, Clone, PartialEq)]
pub struct ZDA {
    /// UTC Time
    pub time: Time,
    /// UTC Date
    pub date: Date,
    /// Local zone hours, 00 to +-13 hrs
    pub local_zone_hours: usize,
    /// Local zone minutes, 00 to 59
    pub local_zone_minutes: usize,
}

impl Default for ZDA {
    fn default() -> Self {
        Self {
            time: Time::default(),
            date: Date::default(),
            local_zone_hours: 0,
            local_zone_minutes: 0,
        }
    }
}

impl Command<ZDA> for ZDA {
    fn parse_command(&self, command: Vec<String>) -> Result<ZDA, crate::types::Error> {
        if command.len() != 6 {
            return Err(Error(format!(
                "Invalid ZDA command length: {}",
                command.join(" ")
            )));
        } else {
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

            let day = command[1].parse::<u8>()?;
            let month = command[2].parse::<u8>()?;
            let year = command[3].parse::<usize>()?;

            let date = Date { day, month, year };

            let local_zone_hours = command[4].parse::<usize>()?;
            let local_zone_minutes = command[5].parse::<usize>()?;

            Ok(ZDA {
                time,
                date,
                local_zone_hours,
                local_zone_minutes,
            })
        }
    }
}

use crate::types::{Command, Error, Time};

/// GBS - GNSS Satellite Fault Detection
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GBS {
    /// UTC Time
    pub time: Time,
    /// Expected error in latitude, in meters
    pub latitude: f64,
    /// Expected error in longitude, in meters
    pub longitude: f64,
    /// Expected error in altitude, in meters
    pub altitude: f64,
    /// ID number of most likely failed satellite
    pub failed_satellite_id: usize,
    /// Probability of missed detection for most likely failed satellite
    pub probability_of_missed_detection: f64,
    /// Estimate of bias in meters on most likely failed satellite
    pub estimate_of_bias: f64,
    /// Standard deviation of bias estimate
    pub standard_deviation_of_bias_estimate: f64,
}

impl Command<GBS> for GBS {
    fn parse_command(&self, command: Vec<String>) -> Result<GBS, Error> {
        let time_split: Vec<&str> = if command[0].contains('.') {
            command[0].split('.').collect()
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

        let latitude = command[1].parse()?;
        let longitude = command[2].parse()?;
        let altitude = command[3].parse()?;
        let failed_satellite_id = command[4].parse()?;
        let probability_of_missed_detection = command[5].parse()?;
        let estimate_of_bias = command[6].parse()?;
        let standard_deviation_of_bias_estimate = command[7].parse()?;

        Ok(GBS {
            time,
            latitude,
            longitude,
            altitude,
            failed_satellite_id,
            probability_of_missed_detection,
            estimate_of_bias,
            standard_deviation_of_bias_estimate,
        })
    }
}

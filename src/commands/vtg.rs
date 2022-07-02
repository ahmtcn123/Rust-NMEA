use crate::types::{Command, Error, ModeIndicator, VTGUnit};

//$GPVTG,81.47,T,,M,0.788,N,1.459,K,A*09

/// VTG ( Course Over Ground and Ground Speed )
#[derive(Debug, Clone, PartialEq)]
pub struct VTG {
    /// Course Over Ground True
    pub course_over_ground_true: Option<f32>,
    /// Course Over Ground True Unit
    pub course_over_ground_unit: char,
    /// Course Over Ground Magnetic
    pub course_over_ground_magnetic: Option<f32>,
    /// Course Over Ground Magnetic Unit
    pub course_over_ground_magnetic_unit: char,
    /// Speed Over Ground First
    pub speed_over_ground_first: Option<f32>,
    /// Speed Over Ground First Unit
    pub speed_over_ground_first_unit: VTGUnit,
    /// Speed Over Ground Second
    pub speed_over_ground_second: Option<f32>,
    /// Speed Over Ground Second Unit
    pub speed_over_ground_second_unit: VTGUnit,
    /// Mode Indicator
    pub mode_indicator: ModeIndicator,
}

impl Default for VTG {
    fn default() -> Self {
        Self {
            course_over_ground_true: Default::default(),
            course_over_ground_unit: Default::default(),
            course_over_ground_magnetic: Default::default(),
            course_over_ground_magnetic_unit: Default::default(),
            speed_over_ground_first: Default::default(),
            speed_over_ground_first_unit: VTGUnit::Knots,
            speed_over_ground_second: Default::default(),
            speed_over_ground_second_unit: VTGUnit::Kmh,
            mode_indicator: ModeIndicator::Autonomous,
        }
    }
}

impl Command<VTG> for VTG {
    fn parse_command(&self, command: Vec<String>) -> Result<VTG, crate::types::Error> {
        if command.len() != 9 {
            return Err(Error(format!(
                "Invalid VTG command length: {}",
                command.join(" ")
            )));
        } else {
            let course_over_ground_true = command[0].parse::<f32>().ok();
            let course_over_ground_unit = match command[1].chars().next() {
                Some(e) => e,
                None => return Err(Error("Invalid course over ground unit".to_string())),
            };

            let course_over_ground_magnetic = command[2].parse::<f32>().ok();
            let course_over_ground_magnetic_unit = match command[3].chars().next() {
                Some(e) => e,
                None => {
                    return Err(Error(
                        "Invalid course over ground magnetic unit".to_string(),
                    ))
                }
            };

            let speed_over_ground_first = command[4].parse::<f32>().ok();
            let speed_over_ground_first_unit = match VTGUnit::from_str(&command[5]) {
                Ok(e) => e,
                Err(_) => return Err(Error("Invalid speed over ground first unit".to_string())),
            };

            let speed_over_ground_second = command[6].parse::<f32>().ok();
            let speed_over_ground_second_unit = match VTGUnit::from_str(&command[7]) {
                Ok(e) => e,
                Err(_) => return Err(Error("Invalid speed over ground second unit".to_string())),
            };

            let mode_indicator = match ModeIndicator::from_str(&command[8]) {
                Ok(e) => e,
                Err(_) => return Err(Error("Invalid mode indicator".to_string())),
            };

            Ok(VTG {
                course_over_ground_true,
                course_over_ground_unit,
                course_over_ground_magnetic,
                course_over_ground_magnetic_unit,
                speed_over_ground_first,
                speed_over_ground_first_unit,
                speed_over_ground_second,
                speed_over_ground_second_unit,
                mode_indicator,
            })
        }
    }
}

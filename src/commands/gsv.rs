use crate::types::{Command, Error};

/// Sattelite struct
#[derive(Debug, Clone, PartialEq)]
pub struct Satellite {
    /// The satellite id
    pub id: Option<usize>,
    /// The satellite elevation in degrees
    pub elevation: Option<usize>,
    /// The satellite azimuth in degrees
    pub azimuth: Option<usize>,
    /// The satellite signal strength in dB
    pub snr: Option<usize>,
}

/// GSV (Sattelite in View)
#[derive(Debug, Clone, PartialEq)]
pub struct GSV {
    /// Total number of GSV Pages
    pub total_pages: usize,
    /// GSV Pages
    pub pages: Vec<GSVPage>,
}

/// GSV Page is child of GSV
#[derive(Debug, Clone, PartialEq)]
pub struct GSVPage {
    /// Number of pages will arrive
    pub total_pages: usize,
    /// Number of page in this page
    pub page_id: usize,
    /// Number of sattelites in this page
    pub number_of_known_satellites_in_view: usize,
    /// Sattelites in this page
    pub satellites: Vec<Satellite>,
    /// Signal ID
    pub signal_id: Option<usize>,
}

impl Default for GSVPage {
    fn default() -> Self {
        Self {
            total_pages: 0,
            page_id: 0,
            number_of_known_satellites_in_view: 0,
            satellites: Vec::new(),
            signal_id: None,
        }
    }
}

impl Command<GSVPage> for GSVPage {
    fn parse_command(&self, command: Vec<String>) -> Result<GSVPage, Error> {
        if command.len() < 4 {
            return Err(Error(format!("Invalid GSV command len: {}", command.len())));
        }

        let total_pages: usize = command[0].parse()?;
        let page_id: usize = command[1].parse()?;
        let number_of_known_satellites_in_view: usize = command[2].parse()?;

        let mut satellites = Vec::new();

        for i in 0..4 {
            let idx = i * 4;

            let id: Option<usize> = if command.len() > idx + 0 {
                match command[idx + 0].parse::<usize>() {
                    Ok(e) => Some(e),
                    Err(_) => None,
                }
            } else {
                None
            };

            let elevation: Option<usize> = if command.len() > idx + 1 {
                match command[idx + 1].parse::<usize>() {
                    Ok(e) => Some(e),
                    Err(_) => None,
                }
            } else {
                None
            };

            let azimuth: Option<usize> = if command.len() > idx + 2 {
                match command[idx + 2].parse::<usize>() {
                    Ok(e) => Some(e),
                    Err(_) => None,
                }
            } else {
                None
            };

            let snr: Option<usize> = if command.len() > idx + 3 {
                match command[idx + 3].parse::<usize>() {
                    Ok(e) => Some(e),
                    Err(_) => None,
                }
            } else {
                None
            };

            satellites.push(Satellite {
                id,
                elevation,
                azimuth,
                snr,
            });
        }
        let signal_id = command[command.len() - 1].parse::<usize>().ok();

        Ok(GSVPage {
            total_pages,
            page_id,
            number_of_known_satellites_in_view,
            satellites,
            signal_id,
        })
    }
}

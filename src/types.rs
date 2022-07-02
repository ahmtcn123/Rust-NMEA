use crate::commands::gga::GGA;
use crate::commands::gll::GLL;
use crate::commands::gsa::GSA;
use crate::commands::gsv::GSVPage;
use crate::commands::rmc::RMC;
use crate::commands::vtg::VTG;
use core::num;

/// Error struct
#[derive(Debug, Clone, PartialEq)]
pub struct Error(pub String);

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error(format!("ParseIntError {}", e.to_string()))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Error {
        Error(format!("ParseFloatError {}", e.to_string()))
    }
}

pub(crate) trait Command<T> {
    fn parse_command(&self, command: Vec<String>) -> Result<T, Error>;
}

/// Command Types
#[derive(Debug, Clone, PartialEq)]
pub enum CommandTypes {
    /// GGA ( Global Positioning System Fix Data )
    GGA(GGA),
    /// GLL ( Geographic Position - Latitude/Longitude )
    GSV(GSVPage),
    /// GLL ( Geographic Position - Latitude/Longitude )
    GLL(GLL),
    /// GSA ( GPS DOP and Active Satellites )
    GSA(GSA),
    /// RMC ( Recommended Minimum Specific GPS/Transit Data )
    RMC(RMC),
    /// VTG ( Course Over Ground and Ground Speed )
    VTG(VTG),
}

impl CommandTypes {
    pub(crate) fn from_str(s: &str) -> Result<CommandTypes, &str> {
        match s {
            "GGA" => Ok(CommandTypes::GGA(GGA::default())),
            "GSV" => Ok(CommandTypes::GSV(GSVPage::default())),
            "GLL" => Ok(CommandTypes::GLL(GLL::default())),
            "GSA" => Ok(CommandTypes::GSA(GSA::default())),
            "VTG" => Ok(CommandTypes::VTG(VTG::default())),
            "RMC" => Ok(CommandTypes::RMC(RMC::default())),
            _ => Err("Invalid command type"),
        }
    }

    pub(crate) fn parse_commands(&mut self, command: Vec<String>) -> Result<CommandTypes, Error> {
        match self {
            CommandTypes::GGA(e) => match e.parse_command(command) {
                Ok(e) => Ok(CommandTypes::GGA(e.clone())),
                Err(e) => Err(e),
            },
            CommandTypes::GSV(e) => match e.parse_command(command) {
                Ok(e) => Ok(CommandTypes::GSV(e.clone())),
                Err(e) => Err(e),
            },
            CommandTypes::GLL(e) => match e.parse_command(command) {
                Ok(e) => Ok(CommandTypes::GLL(e.clone())),
                Err(e) => Err(e),
            },
            CommandTypes::GSA(e) => match e.parse_command(command) {
                Ok(e) => Ok(CommandTypes::GSA(e.clone())),
                Err(e) => Err(e),
            },
            CommandTypes::VTG(e) => match e.parse_command(command) {
                Ok(e) => Ok(CommandTypes::VTG(e.clone())),
                Err(e) => Err(e),
            },
            CommandTypes::RMC(e) => match e.parse_command(command) {
                Ok(e) => Ok(CommandTypes::RMC(e.clone())),
                Err(e) => Err(e),
            },
        }
    }
}

/// Command Status
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// Valid Data
    Valid,
    /// Invalid Data
    Invalid,
}

impl Status {
    pub(crate) fn from_str(s: &str) -> Result<Status, &str> {
        match s {
            "A" => Ok(Status::Valid),
            "V" => Ok(Status::Invalid),
            _ => Err("Invalid status"),
        }
    }
}

/// Time struct
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Time {
    /// UTC hour
    pub hour: u8,
    /// UTC minute
    pub minute: u8,
    /// UTC second
    pub second: u8,
    /// Decimal Seconds
    pub decimal_seconds: u8,
}

/// Date struct
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Date {
    /// UTC year
    pub year: usize,
    /// UTC month
    pub month: u8,
    /// UTC day
    pub day: u8,
}

/// Cordinate struct
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cordinate {
    /// Cordinate in degrees
    pub degree: usize,
    /// Cordinate in minutes
    pub minute: f32,
}

/// GGA command status
#[derive(Debug, Clone, PartialEq)]
pub enum GGAStatus {
    /// Invalid status
    Invalid,
    /// 2D/3D
    S2d3D,
    /// DGNSS
    Dgnss,
    /// Fixed RTK
    FixedRtk,
    /// Float RTK
    FloatRtk,
    /// (dead reckoning)
    DeadReckoning,
}

/// ModeIndicator struct
#[derive(Debug, Clone, PartialEq)]
pub enum ModeIndicator {
    /// Autonomous mode
    Autonomous,
    /// Differential mode
    Differential,
    /// Dead reckoning mode
    DeadReckoning,
    /// None
    None,
}

impl ModeIndicator {
    pub(crate) fn from_str(s: &str) -> Result<ModeIndicator, &str> {
        match s {
            "A" => Ok(ModeIndicator::Autonomous),
            "D" => Ok(ModeIndicator::Differential),
            "E" => Ok(ModeIndicator::DeadReckoning),
            "N" => Ok(ModeIndicator::None),
            _ => Err("Invalid mode indicator"),
        }
    }
}

/// GSA command operation mode
#[derive(Debug, Clone, PartialEq)]
pub enum GSAOperationMode {
    /// Manual mode
    Manual,
    /// Automatic mode (2D/3D)
    Automatic,
}

impl GSAOperationMode {
    pub(crate) fn from_str(s: &str) -> Result<GSAOperationMode, &str> {
        match s {
            "M" => Ok(GSAOperationMode::Manual),
            "A" => Ok(GSAOperationMode::Automatic),
            _ => Err("Invalid operation mode"),
        }
    }
}

/// Navigation Mode struct
#[derive(Debug, Clone, PartialEq)]
pub enum NavigationMode {
    /// No fix
    NoFix,
    /// 2D fix
    Fix2D,
    /// 3D fix
    Fix3D,
}

impl NavigationMode {
    pub(crate) fn from_str(s: &str) -> Result<NavigationMode, &str> {
        match s {
            "1" => Ok(NavigationMode::NoFix),
            "2" => Ok(NavigationMode::Fix2D),
            "3" => Ok(NavigationMode::Fix3D),
            _ => Err("Invalid navigation mode"),
        }
    }
}

/// VTG command speed unit
#[derive(Debug, Clone, PartialEq)]
pub enum VTGUnit {
    /// Knots
    Knots,
    /// Kilometers per hour
    Kmh,
    /// Miles per hour
    Mph,
}

impl VTGUnit {
    pub(crate) fn from_str(s: &str) -> Result<VTGUnit, &str> {
        match s {
            "N" => Ok(VTGUnit::Knots),
            "K" => Ok(VTGUnit::Kmh),
            "M" => Ok(VTGUnit::Mph),
            _ => Err("Invalid VTG unit"),
        }
    }
}

/// Navigation Status struct
#[derive(Debug, Clone, PartialEq)]
pub enum NavigationalStatus {
    /// Safe route
    Safe,
    /// Route is OK but warning
    Caution,
    /// Unsafe route
    Unsafe,
    /// Not valid
    NotValid,
}

impl NavigationalStatus {
    pub(crate) fn from_str(s: &str) -> Result<NavigationalStatus, &str> {
        match s {
            "S" => Ok(NavigationalStatus::Safe),
            "C" => Ok(NavigationalStatus::Caution),
            "U" => Ok(NavigationalStatus::Unsafe),
            "V" => Ok(NavigationalStatus::NotValid),
            _ => Err("Invalid navigational status"),
        }
    }
}

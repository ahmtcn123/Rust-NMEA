#[derive(Debug)]
pub struct MagneticVariation {
    value: f64,
    turn: String
}

#[derive(Debug)]
pub enum Fix {
    Invalid(i8),
    GpsFix(i8),
    DGpsFix(i8),
    PpsFix(i8),
    RealTimeKinematic(i8),
    FloatRTK(i8),
    Estimated(i8),
    ManualInputMode(i8),
    SimulationMode(i8)
}

#[derive(Debug)]
pub struct Cordinate {
    degree: i8,
    minute: f64,
    turn: String
}

#[derive(Debug)]
pub struct GGAData {
    time: String,
    lat: Cordinate,
    lon: Cordinate,
    fix: Fix,
    number_of_satellites: isize,
    horizontal_dilution_of_position: f64,
    altitude: f64,
    height_of_geoid: f64
}

#[derive(Debug)]
pub struct GLLData {
    lat: Cordinate,
    lon: Cordinate,
    fix_taken: String,
    data: String
}

#[derive(Debug)]
pub struct GSVData {
    number_of_sentences: isize,
    sentence: isize,
    number_of_satellites_in_view: isize,
    satellites_prn_number: isize,
    elevation_degrees: isize,
    azimuth_degrees: isize,
    snr: isize,
}

#[derive(Debug)]
pub struct RMCData {
    fix_taken: String,
    status: String,
    lat: Cordinate,
    lon: Cordinate,
    speed_in_knots: f64,
    track_angle_in_degrees: f64,
    date: String,
    magnetic_variation: MagneticVariation 
}

#[derive(Debug)]
pub struct VTGData {
    track: f64,
    magnetic_track: f64,
    speed_in_knots: f64,
    speed_in_km: f64,
}

#[derive(Debug)]
pub enum Result {
    GGA(GGAData),
    GSV(GSVData),
    GLL(GLLData),
    RMC(RMCData),
    VTG(VTGData),
    WrongData(bool)
}

pub fn parse(arrived: String) -> Result {
        let code: String = arrived[3..6].to_string();
        let data : String = arrived[7..].to_string();
        
        if code == "GGA" {
            let latitude_data = data.split(",").collect::<Vec<&str>>()[1].to_string();
            let longitude_data = data.split(",").collect::<Vec<&str>>()[3].to_string();
        

            let latitude_degree: i8 = latitude_data[..2].parse().unwrap();
            let latitude_minute: f64 = latitude_data[2..].parse().unwrap();

            let latitude  = Cordinate {
                degree: latitude_degree,
                minute: latitude_minute,
                turn: data.split(",").collect::<Vec<&str>>()[2].to_string()
            };

            let longitude_degree: i8 = longitude_data[..3].parse().unwrap();
            let longitude_minute: f64 = longitude_data[3..].parse().unwrap();

            let longitude  = Cordinate {
                degree: longitude_degree,
                minute: longitude_minute,
                turn: data.split(",").collect::<Vec<&str>>()[4].to_string()
            };
            
            let time_data : String = data.split(",").collect::<Vec<&str>>()[0].to_string();
            let mut time : String = "".to_string();
            let hour : String = time_data[..2].to_string();
            let minute : String = time_data[2..4].to_string();
            let seconds : String = time_data[4..6].to_string();
            time.push_str(&hour);
            time.push_str(":");
            time.push_str(&minute);
            time.push_str(":");
            time.push_str(&seconds);
            
            let fix_data = data.split(",").collect::<Vec<&str>>()[5];
            let number_of_satellites: isize = data.split(",").collect::<Vec<&str>>()[6].parse().unwrap();
            let horizontal_dilution_of_position: f64 = data.split(",").collect::<Vec<&str>>()[7].parse().unwrap();
            let altitude:f64 = data.split(",").collect::<Vec<&str>>()[8].parse().unwrap();
            let height_of_geoid:f64 = data.split(",").collect::<Vec<&str>>()[10].parse().unwrap();

            // $GPGGA,173003.00,4110.10881,N,02859.40944,E,1,05,2.77,116.8,M,37.3,M,,*5B

            return Result::GGA(GGAData {
                time,
                lat: latitude,
                lon: longitude,
                fix: match fix_data {
                    "0" => Fix::Invalid(0),
                    "1" => Fix::GpsFix(1),
                    "2" => Fix::DGpsFix(2),
                    "3" => Fix::PpsFix(3),
                    "4" => Fix::RealTimeKinematic(4),
                    "5" => Fix::FloatRTK(5),
                    "6" => Fix::Estimated(6),
                    "7" => Fix::ManualInputMode(7),
                    "8" => Fix::SimulationMode(8),
                    _ => Fix::Invalid(0)
                },
                number_of_satellites,
                horizontal_dilution_of_position,
                altitude,
                height_of_geoid
            });
        } else if code == "GSV" {
            let number_of_sentences:isize = data.split(",").collect::<Vec<&str>>()[0].parse().unwrap();
            let sentence:isize = data.split(",").collect::<Vec<&str>>()[1].parse().unwrap();
            let number_of_satellites_in_view:isize = data.split(",").collect::<Vec<&str>>()[2].parse().unwrap();
            let satellites_prn_number:isize = data.split(",").collect::<Vec<&str>>()[3].parse().unwrap();
            let elevation_degrees:isize = data.split(",").collect::<Vec<&str>>()[4].parse().unwrap();
            let azimuth_degrees:isize = data.split(",").collect::<Vec<&str>>()[5].parse().unwrap();
            let snr :isize = data.split(",").collect::<Vec<&str>>()[6].parse().unwrap();

            return Result::GSV(GSVData {
                number_of_sentences,
                sentence,
                number_of_satellites_in_view,
                satellites_prn_number,
                elevation_degrees,
                azimuth_degrees,
                snr
            })
        } else if code == "GLL" {
            let latitude_data = data.split(",").collect::<Vec<&str>>()[0].to_string();
            let longitude_data = data.split(",").collect::<Vec<&str>>()[2].to_string();
            
            let latitude_degree: i8 = latitude_data[..2].parse().unwrap();
            let latitude_minute: f64 = latitude_data[2..].parse().unwrap();

            let lat  = Cordinate {
                degree: latitude_degree,
                minute: latitude_minute,
                turn: data.split(",").collect::<Vec<&str>>()[1].to_string()
            };

            let longitude_degree: i8 = longitude_data[..2].parse().unwrap();
            let longitude_minute: f64 = longitude_data[2..].parse().unwrap();

            let lon  = Cordinate {
                degree: longitude_degree,
                minute: longitude_minute,
                turn: data.split(",").collect::<Vec<&str>>()[3].to_string()
            };
            
            let time_data : String = data.split(",").collect::<Vec<&str>>()[4].to_string();
            let mut time : String = "".to_string();
            let hour : String = time_data[..2].to_string();
            let minute : String = time_data[2..4].to_string();
            let seconds : String = time_data[4..6].to_string();
            time.push_str(&hour);
            time.push_str(":");
            time.push_str(&minute);
            time.push_str(":");
            time.push_str(&seconds);
            return Result::GLL(GLLData {
                lat,
                lon,
                fix_taken: time,
                data: data.split(",").collect::<Vec<&str>>()[5].to_string()
            })
        } else if code == "RMC" {
            let latitude_data = data.split(",").collect::<Vec<&str>>()[2].to_string();
            let longitude_data = data.split(",").collect::<Vec<&str>>()[4].to_string();
        

            let latitude_degree: i8 = latitude_data[..2].parse().unwrap();
            let latitude_minute: f64 = latitude_data[2..].parse().unwrap();

            let lat  = Cordinate {
                degree: latitude_degree,
                minute: latitude_minute,
                turn: data.split(",").collect::<Vec<&str>>()[3].to_string()
            };

            let longitude_degree: i8 = longitude_data[..3].parse().unwrap();
            let longitude_minute: f64 = longitude_data[3..].parse().unwrap();

            let lon  = Cordinate {
                degree: longitude_degree,
                minute: longitude_minute,
                turn: data.split(",").collect::<Vec<&str>>()[5].to_string()
            };
            
            let time_data : String = data.split(",").collect::<Vec<&str>>()[0].to_string();
            let mut time : String = "".to_string();
            let hour : String = time_data[..2].to_string();
            let minute : String = time_data[2..4].to_string();
            let seconds : String = time_data[4..6].to_string();
            time.push_str(&hour);
            time.push_str(":");
            time.push_str(&minute);
            time.push_str(":");
            time.push_str(&seconds);

            let date_data : String = data.split(",").collect::<Vec<&str>>()[8].to_string();
            let mut date : String = "".to_string();
            let day : String = date_data[..2].to_string();
            let month : String = date_data[2..4].to_string();
            let year : String = date_data[4..6].to_string();
            date.push_str(&day);
            date.push_str(".");
            date.push_str(&month);
            date.push_str(".");
            date.push_str(&year);
            
            let magnetic_variation = MagneticVariation {
                value: data.split(",").collect::<Vec<&str>>()[9].parse().unwrap(),
                turn: data.split(",").collect::<Vec<&str>>()[10].to_string()
            };

            return Result::RMC(RMCData {
                fix_taken: time,
                status: data.split(",").collect::<Vec<&str>>()[1].to_string(),
                lat,
                lon,
                speed_in_knots: data.split(",").collect::<Vec<&str>>()[6].parse().unwrap(),
                track_angle_in_degrees: data.split(",").collect::<Vec<&str>>()[7].parse().unwrap(),
                date,
                magnetic_variation
            });
        } else if code == "VTG" {
            return Result::VTG(VTGData {
                track: data.split(",").collect::<Vec<&str>>()[0].parse().unwrap(),
                magnetic_track: data.split(",").collect::<Vec<&str>>()[2].parse().unwrap(),
                speed_in_knots: data.split(",").collect::<Vec<&str>>()[4].parse().unwrap(),
                speed_in_km: data.split(",").collect::<Vec<&str>>()[6].parse().unwrap(),
            })
        } else {
            return Result::WrongData(true)
        }
}
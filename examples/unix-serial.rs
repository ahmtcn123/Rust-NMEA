// Open serial device as file in unix;
// USAGE:
// unix-serial /dev/cu.usbserial-0001

// Use `ls /dev/{tty,cu}.*` to find the correct device
// Tested on macos with UART usb adapter with UBLOX NEO-6M

use rust_nmea::parser;
use std::io::Read;

fn main() {
    // Read arguments
    let args: Vec<String> = std::env::args().collect();
    let device = &args[1];
    println!("Using device: {}", device);
    let mut serial_file = std::fs::File::open(device).unwrap();
    let mut carriage_return = false;
    let mut line = String::new();

    loop {
        //Read char
        let mut buf = [0u8; 1];
        serial_file.read(&mut buf).unwrap();
        let char = buf[0] as char;

        // If carriage return is detected ignore and make `carriage_return` false
        if char == '\r' {
            carriage_return = true;
        } else if char == '\n' && carriage_return {
            //If carriage return is detected and char is `\n` parse next line
            let parsed_line = parser::Parser::parse_line(&line);
            match parsed_line {
                Ok(parsed) => {
                    println!("{:?}", parsed);
                }
                Err(_) => (),
            }
            line = "".to_string();
            carriage_return = false;
        } else if char != '\0' {
            line.push(char);
        }
    }
}

use rust_nmea::parser;

fn main() {
    let line = "$GPGGA,161009.00,1122.20418,N,02339.35234,E,1,08,1.09,11.5,M,11.3,M,,*62";
    let parsed_line = parser::Parser::parse_line(line);

    println!("{:#?}", parsed_line)
}
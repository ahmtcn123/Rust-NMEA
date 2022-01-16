# Rust-GPS [Needs-Rebuild]
GPS package parser according to NMEA standards

| Package Type                                           | Implemented  |
| ------------------------------------------------------ | ------------ |
| [AAM](AAMLink) (Waypoint Arrival Alarm)                |      ❌      |
| [ALM](ALMLink) (Almanac Data)                          |      ❌      |
| [APB](APBLink) (Auto Pilot B Sentence)                 |      ❌      |
| [BOD](BODLink) (Bearing Origin To Destination)         |      ❌      |
| [BWC](BWCLink) (Bearing Using Great Circle Route)      |      ❌      |
| [GGA](GGALink) (Fix Information)                       |      ✅      |
| [GLL](GLLLink) (Lat/Lon Data)                          |      ✅      |
| [GSA](GSALink) (Overall Sattelite Data)                |      ❌      |
| [GSV](GSVLink) (Detailed Sattelite Data)               |      ✅      |
| [MSK](MSKLink) (Send Control For A Beacon Receiver)    |      ❌      |
| [MSS](MSSLink) (Beacon Receiver Status Information)    |      ❌      |
| [RMB](RMBLink) (Recommended Navigation Data For GPS)   |      ❌      |
| [RMC](RMCLink) (Recommended Minimum Data For GPS)      |      ❌      |
| [RTE](RTELink) (Route Message)                         |      ❌      |
| [VTG](VTGLink) (Vector Track An Speed Over The Ground) |      ✅      |
| [WPL](WPLLink) (Waypoint Location Information)         |      ❌      |
| [XTE](XTELink) (Measured Cross Track Error)            |      ❌      |
| [ZTA](ZTALink) (Date And Time)                         |      ❌      |

Important: This libary does not provide SerialPort

[AAMLink]: http://www.gpsinformation.org/dale/nmea.htm#AAM
[ALMLink]: http://www.gpsinformation.org/dale/nmea.htm#ALM
[APBLink]: http://www.gpsinformation.org/dale/nmea.htm#APB
[BODLink]: http://www.gpsinformation.org/dale/nmea.htm#BOD
[BWCLink]: http://www.gpsinformation.org/dale/nmea.htm#BWC
[GGALink]: http://www.gpsinformation.org/dale/nmea.htm#GGA
[GLLLink]: http://www.gpsinformation.org/dale/nmea.htm#GLL
[GSALink]: http://www.gpsinformation.org/dale/nmea.htm#GSA
[GSVLink]: http://www.gpsinformation.org/dale/nmea.htm#GSV
[MSKLink]: http://www.gpsinformation.org/dale/nmea.htm#MSK
[MSSLink]: http://www.gpsinformation.org/dale/nmea.htm#MSS
[RMBLink]: http://www.gpsinformation.org/dale/nmea.htm#RMB
[RMCLink]: http://www.gpsinformation.org/dale/nmea.htm#RMC
[RTELink]: http://www.gpsinformation.org/dale/nmea.htm#RTE
[VTGLink]: http://www.gpsinformation.org/dale/nmea.htm#VTG
[WPLLink]: http://www.gpsinformation.org/dale/nmea.htm#WPL
[XTELink]: http://www.gpsinformation.org/dale/nmea.htm#XTE
[ZTALink]: http://www.gpsinformation.org/dale/nmea.htm#ZTA

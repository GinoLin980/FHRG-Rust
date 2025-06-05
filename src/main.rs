use crate::udp_decode::DataType;

mod udp_decode;
mod udp_receive;

fn main() {
    let ips: [[u8; 4]; 1] = [[127, 0, 0, 1]];
    let ports = [8000];

    let socket = udp_receive::connect(&udp_receive::udp_connectable(&ips, &ports));

    loop {
        let data = udp_receive::receive(&socket)
            .expect("error receiving from udp");

        let race = match data.get("IsRaceOn") {
            Some(DataType::Int(v)) => *v,
            _ => {
                eprintln!("error when getting IsRaceOn");
                0
            }
        };
        let speed = match data.get("Speed") {
            Some(DataType::Float(v)) => *v,
            _ => {
                eprintln!("error when getting Speed");
                0.0
            }
        };
        let gear = match data.get("Gear") {
            Some(DataType::UByte(v)) => *v,
            _ => {
                eprintln!("error when getting Gear");
                0
            }
        };

        print!("{}[2J", 27 as char);
        println!("IsRaceOn: {}", race);
        println!("Speed: {}", speed * 3.6);
        println!("Gear: {}", gear);
    }
}

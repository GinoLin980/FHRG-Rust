mod udp_decode;
mod udp_receive;
mod data_schema;

// in Forza, Speed returns Meter per second
const MPS_TO_KMH: f32 = 3.62;

fn main() {
    let ips: [[u8; 4]; 1] = [[127, 0, 0, 1]];
    let ports = [8000];

    let socket = udp_receive::connect(&udp_receive::udp_connectable(&ips, &ports));

    loop {
        let data = udp_receive::receive(&socket)
            .expect("error receiving from udp");

        let speed: f32 = data.get("Speed").expect("Can't get Speed").try_into().expect("Can't convert Speed");
        let race: i32 = data.get("IsRaceOn").expect("Can't get IsRaceOn").try_into().expect("Can't convert Race");
        let gear: i32 = data.get("Gear").expect("Can't get Gear").try_into().expect("Can't convert Gear");

        print!("{}[2J", 27 as char);
        println!("IsRaceOn: {}", race);
        println!("Speed: {}", speed * MPS_TO_KMH);
        println!("Gear: {}", gear);
    }
}

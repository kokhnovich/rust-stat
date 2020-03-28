use std::time::Duration;

use std::fs;

use fleetspeak::Packet;

fn main() -> std::io::Result<()> {
    fleetspeak::startup("0.0.1")?;

    loop {
        let packet = fleetspeak::collect(Duration::from_secs(1))?;

        let request: String = packet.data;
        // let attr = fs::metadata(request)?;
        let response: String = format!("Hello {}!", request);

        fleetspeak::send(Packet {
            service: String::from("stat"),
            kind: None,
            data: response,
        })?;
    }
}

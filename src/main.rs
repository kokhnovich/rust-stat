use fleetspeak::Packet;
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::time::Duration;

#[test]
fn stat_proto_structs_existance() {
    assert!(fs::metadata(concat!(env!("OUT_DIR"), "/stat.rs")).is_ok());
}

pub mod stat {
    include!(concat!(env!("OUT_DIR"), "/stat.rs"));
}

fn SendData(req:stat::Request, meta: fs::Metadata) -> std::io::Result<()> {
    let resp = stat::Response {
        path: req.path,
        size: meta.len(),
        mode: meta.st_mode() as u64,
        extra: Some(stat::response::Extra {
            blocks: meta.st_blocks(),
            io_blocks: meta.st_blksize(),
            inode: meta.st_ino(),
            links: meta.st_nlink(),
        }),
    };
    fleetspeak::send(Packet {
        service: String::from("stat"),
        kind: Some("response".to_string()),
        data: resp,
    })?;
    Ok(())
}

fn SendError(e: std::io::Error) -> std::io::Result<()> {
    fleetspeak::send(Packet {
        service: String::from("stat"),
        kind: Some("error".to_string()),
        data: stat::Error {
            what: e.to_string(),
        },
    })?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    fleetspeak::startup("0.1.1")?;

    loop {
        let packet = fleetspeak::collect(Duration::from_secs(1))?;

        let req: stat::Request = packet.data;

        match fs::metadata(&req.path) {
            Ok(meta) => SendData(req, meta),
            Err(e) => SendError(e),
        }?;
    }
}

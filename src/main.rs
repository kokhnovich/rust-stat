use std::time::Duration;
use std::fs;
use std::os::linux::fs::MetadataExt;
use fleetspeak::Packet;
use std::path::Path;


mod stat;
use crate::stat::Request;
use crate::stat::Response;
use crate::stat::response;


fn main() -> std::io::Result<()> {
    fleetspeak::startup("0.1.1")?;

    loop {
        let packet = fleetspeak::collect(Duration::from_secs(1))?;

        let req: Request = packet.data;
	if Path::new(&req.path).exists() {
            let meta = fs::metadata(&req.path)?;
            let resp = Response { path: req.path, size: meta.len(), mode: meta.st_mode() as u64,
                                  extra: Some(response::Extra { blocks: meta.st_blocks(),
                                                            io_blocks: meta.st_blksize(),
                                                            inode: meta.st_ino(),
                                                            links: meta.st_nlink() } ),
                                  errors: false };
            fleetspeak::send(Packet {
                service: String::from("stat"),
                kind: None,
                data: resp,
            })?;
        } else {
            fleetspeak::send(Packet {
                service: String::from("stat"),
                kind: Some("no such file error".to_string()),
                data: Response::default(),
            })?;
        }
    }
}

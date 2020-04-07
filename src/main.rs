use fleetspeak::Packet;
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::time::Duration;

pub mod stat {
    include!(concat!(env!("OUT_DIR"), "/stat.rs"));
}

fn get_data(path: String) -> std::io::Result<stat::Response> {
    let meta = fs::metadata(&path)?;
    let data = Some(stat::response::Extra {
        blocks: meta.st_blocks(),
        io_blocks: meta.st_blksize(),
        inode: meta.st_ino(),
        links: meta.st_nlink(),
        dev: meta.st_dev(),
        mode: meta.st_mode(),
        uid: meta.st_uid(),
        gid: meta.st_gid(),
        atime: meta.st_atime(),
        mtime: meta.st_mtime(),
        ctime: meta.st_ctime(),
    });
    let resp = stat::Response {
        path: path,
        size: meta.len(),
        mode: meta.st_mode() as u64,
        extra: data,
    };
    Ok(resp)
}

fn send_data(data: stat::Response) -> std::io::Result<()> {
    fleetspeak::send(Packet {
        service: String::from("stat"),
        kind: Some("response".to_string()),
        data: data,
    })?;
    Ok(())
}

fn send_error(e: std::io::Error) -> std::io::Result<()> {
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

        match get_data(req.path) {
            Ok(data_) => send_data(data_),
            Err(e) => send_error(e),
        }?;
    }
}

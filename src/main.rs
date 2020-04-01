use fleetspeak::Packet;
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::time::Duration;

#[test]
fn stat_proto_structs_existance() {
    assert!(fs::metadata(concat!(env!("OUT_DIR"), "/stat.rs")).is_ok());
}

#[test]
fn test_fs_metadata() {
    assert!(fs::metadata("/").is_ok());
    assert!(!fs::metadata("/agfagdsfhags").is_ok());
}

pub mod stat {
    include!(concat!(env!("OUT_DIR"), "/stat.rs"));
}

fn send_data(req_path: String, meta: fs::Metadata) -> std::io::Result<()> {
    let resp = stat::Response {
        path: req_path,
        size: meta.len(),
        mode: meta.st_mode() as u64,
        extra: Some(stat::response::Extra {
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
        }),
    };
    fleetspeak::send(Packet {
        service: String::from("stat"),
        kind: Some("response".to_string()),
        data: resp,
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

        match fs::metadata(&req.path) {
            Ok(meta) => send_data(req.path, meta),
            Err(e) => send_error(e),
        }?;
    }
}

use std::time::Duration;

use std::process::Command;

use std::fs;
use std::os::linux::fs::MetadataExt;

use fleetspeak::Packet;

// @TODO copy to separate file
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(string, tag="1")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(string, tag="1")]
    pub path: std::string::String,
    #[prost(uint64, tag="2")]
    pub size: u64,
    #[prost(uint64, tag="3")]
    pub mode: u64,
    #[prost(message, optional, tag="4")]
    pub extra: ::std::option::Option<response::Extra>,
}
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Extra {
        #[prost(uint64, tag="1")]
        pub blocks: u64,
        #[prost(uint64, tag="2")]
        pub io_blocks: u64,
        #[prost(uint64, tag="3")]
        pub inode: u64,
        #[prost(uint64, tag="4")]
        pub links: u64,
    }
}


fn main() -> std::io::Result<()> {
    fleetspeak::startup("0.1.1")?;

    loop {
        let packet = fleetspeak::collect(Duration::from_secs(1))?;

        let req: Request = packet.data;
        let meta = fs::metadata(&req.path)?;
        // let resp: Response = format!("Hello {}!", meta.st_ino());
	// let extra = Extra {..., inode: meta.st_ino() };
        let resp = Response { path: req.path, size: meta.len(), mode: meta.st_mode() as u64,
                              extra: Some(response::Extra { blocks: meta.st_blocks(),
                                                            io_blocks: meta.st_blksize(),
                                                            inode: meta.st_ino(),
                                                            links: meta.st_nlink() } ) };
        fleetspeak::send(Packet {
            service: String::from("stat"),
            kind: None,
            data: resp,
        })?;
    }
}

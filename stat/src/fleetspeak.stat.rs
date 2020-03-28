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
        #[prost(string, tag="1")]
        pub file_name: std::string::String,
        #[prost(uint64, tag="2")]
        pub size: u64,
        #[prost(uint64, tag="3")]
        pub blocks: u64,
        #[prost(uint64, tag="4")]
        pub io_blocks: u64,
        /// TODO add all
        #[prost(uint64, tag="5")]
        pub inode: u64,
    }
}

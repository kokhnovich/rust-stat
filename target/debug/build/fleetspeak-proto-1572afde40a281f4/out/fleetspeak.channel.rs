/// Optional first message sent through a channel when it is created. It is meant
/// to contain info about the process that the other end of the channel might
/// find useful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartupData {
    /// Self-reported PID.
    #[prost(int64, tag="1")]
    pub pid: i64,
    /// Self-reported service version string.
    #[prost(string, tag="2")]
    pub version: std::string::String,
}

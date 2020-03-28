/// An Address identifies the source or destination of a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    /// The client_id, if the address refers to a service on a client. If unset,
    /// the address refers to a service on the server.
    #[prost(bytes, tag="1")]
    pub client_id: std::vec::Vec<u8>,
    /// The name of the fleetspeak service which sent or should receive the
    /// message.  Required.
    #[prost(string, tag="2")]
    pub service_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationInfo {
    #[prost(map="string, string", tag="1")]
    pub tags: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// A hash based on origin and origin_message_id. It is set by the fleetspeak
    /// system on message intake and may be used for deduplication.
    #[prost(bytes, tag="1")]
    pub message_id: std::vec::Vec<u8>,
    /// The source of the messages. Required.
    #[prost(message, optional, tag="2")]
    pub source: ::std::option::Option<Address>,
    /// An sequence of bytes set by the source in a way to uniquely identify the
    /// message among all messages with the same origin.
    #[prost(bytes, tag="3")]
    pub source_message_id: std::vec::Vec<u8>,
    /// The destination of the message. Required.
    #[prost(message, optional, tag="4")]
    pub destination: ::std::option::Option<Address>,
    /// The type of message. This field is mostly opaque to the Fleetspeak system,
    /// but can be used for some statistics gathering. It is recommended that each
    /// service define a static collection of short readable message types and
    /// dispatch according to this when processing messages. e.g. "ResourceUsage",
    /// "StdOutputData".
    #[prost(string, tag="5")]
    pub message_type: std::string::String,
    /// Set when the message enters the FS system.
    #[prost(message, optional, tag="6")]
    pub creation_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The data associated with this request, accepted types are determined by the
    /// service and may depend on message_type. Not typically stored after the
    /// message is processed.
    #[prost(message, optional, tag="7")]
    pub data: ::std::option::Option<::prost_types::Any>,
    /// Additional validation information, set by on the server by the (optional)
    /// authorizer component based on WrappedContactData.validators, etc.
    #[prost(message, optional, tag="8")]
    pub validation_info: ::std::option::Option<ValidationInfo>,
    /// The result of processing the message. Set once processing has finished.
    #[prost(message, optional, tag="9")]
    pub result: ::std::option::Option<MessageResult>,
    #[prost(enumeration="message::Priority", tag="10")]
    pub priority: i32,
    /// A background message does not count as activity when deciding how fast to
    /// poll the server. This flag should be set on messages which are unlikely to
    /// trigger additional activity.
    #[prost(bool, tag="11")]
    pub background: bool,
    /// Optional debugging information provided by the originator of the
    /// message. Fleetspeak stores this information along with the message for
    /// later retrieval, but doesn't really do anything else with it.
    #[prost(message, optional, tag="12")]
    pub annotations: ::std::option::Option<Annotations>,
}
pub mod message {
    /// The message priority. The primary effect is on the ordering of messages
    /// sent from the client to the server.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority {
        /// Out of order, but results in a sensible default.
        Medium = 0,
        Low = 1,
        High = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageResult {
    /// The time that processing finished.
    #[prost(message, optional, tag="2")]
    pub processed_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Set when processing ended with a permanent failure.
    #[prost(bool, tag="3")]
    pub failed: bool,
    /// A human readable error message, normally set when failed is true.
    #[prost(string, tag="4")]
    pub failed_reason: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotations {
    #[prost(message, repeated, tag="1")]
    pub entries: ::std::vec::Vec<annotations::Entry>,
}
pub mod annotations {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(string, tag="1")]
        pub key: std::string::String,
        #[prost(string, tag="2")]
        pub value: std::string::String,
    }
}
/// A Label is a tag assigned to a client by a plugin. Primary use is to limit
/// broadcasts to specific clients.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// The service which set this label.
    #[prost(string, tag="1")]
    pub service_name: std::string::String,
    /// A free form tag choosen by the setting plugin.
    #[prost(string, tag="2")]
    pub label: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    /// A chain of ASN.1 DER encoded x509 certificates.
    #[prost(bytes, repeated, tag="1")]
    pub certificate: ::std::vec::Vec<std::vec::Vec<u8>>,
    /// Indicates the choice of signature algorithm, a constant from
    /// https://golang.org/pkg/crypto/x509/#SignatureAlgorithm
    #[prost(int32, tag="2")]
    pub algorithm: i32,
    /// A signature of the validated data, it should be consistent with both the
    /// algorithm choice and the first element of the certificate chain.
    #[prost(bytes, tag="3")]
    pub signature: std::vec::Vec<u8>,
}
/// A WrappedContactData is provided by the client to the server with every
/// contact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrappedContactData {
    /// A serialized ContactData.
    #[prost(bytes, tag="1")]
    pub contact_data: std::vec::Vec<u8>,
    /// Optional extra signatures validating
    #[prost(message, repeated, tag="2")]
    pub signatures: ::std::vec::Vec<Signature>,
    /// contact_data.
    ///
    /// The labels that the client claims to
    #[prost(string, repeated, tag="3")]
    pub client_labels: ::std::vec::Vec<std::string::String>,
}
/// On every contact, the client and server exchange ContactData messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactData {
    /// During every contact, the server passes a random sequencing_nonce to the
    /// client, and the client provides the sequencing_nonce to the server during
    /// the next contact.
    #[prost(uint64, tag="1")]
    pub sequencing_nonce: u64,
    #[prost(message, repeated, tag="2")]
    pub messages: ::std::vec::Vec<Message>,
    /// Records the client's current time setting, as of the creation of this
    /// ContactData. Only set by the client.
    #[prost(message, optional, tag="3")]
    pub client_clock: ::std::option::Option<::prost_types::Timestamp>,
    /// If set, acknowledges receipt of a streamed WrappedContactData. This is used
    /// for control purposes during streaming connections.
    #[prost(uint64, tag="4")]
    pub ack_index: u64,
    /// If set, indicates that the connection is shutting down and no more data
    /// will be sent.
    #[prost(bool, tag="5")]
    pub done_sending: bool,
    /// Set by the client to indicate how many messages it is willing to accept for
    /// each installed service. After the first exchange of an streaming connection, it
    /// represents the number of additional messages that will be accepted.
    #[prost(map="string, uint64", tag="6")]
    pub allowed_messages: ::std::collections::HashMap<std::string::String, u64>,
}
/// An empty message, typically used as a trivial RPC response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyMessage {
}

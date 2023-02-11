// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Accounts {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Account>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symcode: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub balance: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Stat>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stat {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symcode: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub max_supply: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub supply: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<TransferEvent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_ordinal: u32,
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symcode: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub precision: u32,
    #[prost(string, tag="6")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub to: ::prost::alloc::string::String,
    #[prost(int64, tag="8")]
    pub amount: i64,
    #[prost(string, tag="9")]
    pub memo: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `antelope.eosio.token.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x83, 0x11, 0x0a, 0x11, 0x65, 0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x61, 0x6e, 0x74, 0x65, 0x6c, 0x6f, 0x70, 0x65,
    0x2e, 0x65, 0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x76, 0x31, 0x1a,
    0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x42, 0x0a, 0x08, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x73, 0x12, 0x36, 0x0a, 0x05,
    0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x61, 0x6e,
    0x74, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x65, 0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x05, 0x69,
    0x74, 0x65, 0x6d, 0x73, 0x22, 0x73, 0x0a, 0x07, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12,
    0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73,
    0x79, 0x6d, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x79,
    0x6d, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12,
    0x18, 0x0a, 0x07, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x22, 0x3c, 0x0a, 0x05, 0x53, 0x74, 0x61,
    0x74, 0x73, 0x12, 0x33, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1d, 0x2e, 0x61, 0x6e, 0x74, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x65, 0x6f, 0x73,
    0x69, 0x6f, 0x2e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x61, 0x74,
    0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x22, 0x8b, 0x01, 0x0a, 0x04, 0x53, 0x74, 0x61, 0x74,
    0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x12, 0x18, 0x0a, 0x07,
    0x73, 0x79, 0x6d, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73,
    0x79, 0x6d, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x12, 0x1d,
    0x0a, 0x0a, 0x6d, 0x61, 0x78, 0x5f, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x09, 0x6d, 0x61, 0x78, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x12, 0x16, 0x0a,
    0x06, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73,
    0x75, 0x70, 0x70, 0x6c, 0x79, 0x22, 0x4e, 0x0a, 0x0e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65,
    0x72, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x3c, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x61, 0x6e, 0x74, 0x65, 0x6c, 0x6f, 0x70,
    0x65, 0x2e, 0x65, 0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x76, 0x31,
    0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x05,
    0x69, 0x74, 0x65, 0x6d, 0x73, 0x22, 0xef, 0x01, 0x0a, 0x0d, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x65, 0x72, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x15, 0x0a, 0x06, 0x74, 0x72, 0x78, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x72, 0x78, 0x49, 0x64, 0x12, 0x25,
    0x0a, 0x0e, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4f, 0x72,
    0x64, 0x69, 0x6e, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12,
    0x18, 0x0a, 0x07, 0x73, 0x79, 0x6d, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x73, 0x79, 0x6d, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x70, 0x72, 0x65,
    0x63, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x70, 0x72,
    0x65, 0x63, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x66, 0x72, 0x6f, 0x6d, 0x12, 0x0e, 0x0a, 0x02, 0x74,
    0x6f, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x74, 0x6f, 0x12, 0x16, 0x0a, 0x06, 0x61,
    0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x03, 0x52, 0x06, 0x61, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x65, 0x6d, 0x6f, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6d, 0x65, 0x6d, 0x6f, 0x4a, 0xe4, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x2b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x07, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07,
    0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x15, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x1e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x0a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0b, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x0b, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x16, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0c, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0c, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x04,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x0b, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x0e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x0e, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e,
    0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x11, 0x00, 0x13, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x11, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x12, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x12, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x12,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x1a, 0x1b, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x15, 0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x03, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x16, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x0b, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x16, 0x17, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x17, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x17, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x18,
    0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x18, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x19, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x19, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x19, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x19, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1a, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1a, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x1d, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1d,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1e, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x1b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1e, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x21,
    0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x21, 0x08, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x22, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03,
    0x23, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x23, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x0b, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x24, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x24, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x25, 0x04,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x25, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x0b, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x04, 0x12, 0x03, 0x26, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x26, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x26, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26,
    0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x27, 0x04, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x27, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x27, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x27, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x06, 0x12, 0x03, 0x28, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x28, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x01, 0x12, 0x03, 0x28,
    0x0b, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03, 0x12, 0x03, 0x28, 0x10, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x07, 0x12, 0x03, 0x29, 0x04, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x07, 0x05, 0x12, 0x03, 0x29, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x07, 0x01, 0x12, 0x03, 0x29, 0x0a, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x29, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x08, 0x12,
    0x03, 0x2a, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x05, 0x12, 0x03, 0x2a,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x01, 0x12, 0x03, 0x2a, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x03, 0x12, 0x03, 0x2a, 0x12, 0x13, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
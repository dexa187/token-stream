// @generated
/// Lines represents an ordered list of lines that have been extracted of a single block. You are
/// free the format each line as you please, the `substream-sink-files` tool does not make any
/// assumption about the content and simply write the content to the current bundle with a trailing
/// new line.
///
/// It is expected that your line do **not** contain a new line character as it will be managed
/// manually by the `substream-sink-files` tool.
///
/// The most common use case is to use CSV or JSONL format for your line. For example, you can
/// extract each transaction out of the block as a single line in JSON format as an object. The
/// `substream-sink-files` will then package them in a bundle for N blocks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lines {
    #[prost(string, repeated, tag="1")]
    pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `sf.substreams.sink.files.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf7, 0x0b, 0x0a, 0x27, 0x73, 0x66, 0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x73, 0x2f, 0x73, 0x69, 0x6e, 0x6b, 0x2f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x76, 0x31,
    0x2f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1b, 0x73, 0x66,
    0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b,
    0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x22, 0x1d, 0x0a, 0x05, 0x4c, 0x69, 0x6e,
    0x65, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x6c, 0x69, 0x6e, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x05, 0x6c, 0x69, 0x6e, 0x65, 0x73, 0x42, 0x5b, 0x5a, 0x59, 0x67, 0x69, 0x74, 0x68,
    0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67,
    0x66, 0x61, 0x73, 0x74, 0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2d,
    0x73, 0x69, 0x6e, 0x6b, 0x2d, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x70, 0x62, 0x2f, 0x73, 0x66,
    0x2f, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2f, 0x73, 0x69, 0x6e, 0x6b,
    0x2f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2f, 0x76, 0x31, 0x3b, 0x70, 0x62, 0x73, 0x69, 0x6e, 0x6b,
    0x66, 0x69, 0x6c, 0x65, 0x73, 0x4a, 0xaa, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x19, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0xe9, 0x03, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x08, 0x00, 0x24, 0x1a, 0xde, 0x03, 0x20, 0x54, 0x68, 0x65, 0x20, 0x60, 0x73, 0x66,
    0x2e, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x73, 0x69, 0x6e, 0x6b,
    0x2e, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x60, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61,
    0x67, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x20, 0x69, 0x6e,
    0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x53, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x61, 0x20, 0x60, 0x6d,
    0x61, 0x70, 0x60, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x0a, 0x20, 0x77, 0x68, 0x6f, 0x73,
    0x65, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x2e,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x60, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73,
    0x2d, 0x73, 0x69, 0x6e, 0x6b, 0x2d, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x60, 0x0a, 0x20, 0x62, 0x69,
    0x6e, 0x61, 0x72, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x63,
    0x6f, 0x6e, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x6d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x27, 0x73, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x63,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20,
    0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a,
    0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x61,
    0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x61,
    0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x60, 0x4c, 0x69, 0x6e, 0x65, 0x73, 0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x73, 0x0a, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x70,
    0x6c, 0x61, 0x69, 0x6e, 0x2d, 0x74, 0x65, 0x78, 0x74, 0x20, 0x22, 0x6c, 0x69, 0x6e, 0x65, 0x22,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x61, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x67, 0x65, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x62, 0x75,
    0x6e, 0x64, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x70,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0a, 0x00, 0x70, 0x0a, 0xc3, 0x05, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x17, 0x00, 0x19, 0x01, 0x1a, 0xb6, 0x05, 0x20, 0x4c, 0x69, 0x6e, 0x65,
    0x73, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20,
    0x6f, 0x72, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20,
    0x62, 0x65, 0x65, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x65, 0x64, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x2e, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x66, 0x72, 0x65, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x20, 0x61, 0x73, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x70, 0x6c, 0x65, 0x61,
    0x73, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x2d, 0x73, 0x69, 0x6e, 0x6b, 0x2d, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x60, 0x20, 0x74,
    0x6f, 0x6f, 0x6c, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6d, 0x61, 0x6b,
    0x65, 0x20, 0x61, 0x6e, 0x79, 0x0a, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x79, 0x20, 0x77,
    0x72, 0x69, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20,
    0x62, 0x75, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x74, 0x72,
    0x61, 0x69, 0x6c, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6c, 0x69, 0x6e, 0x65,
    0x2e, 0x0a, 0x0a, 0x20, 0x49, 0x74, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x6c, 0x69, 0x6e,
    0x65, 0x20, 0x64, 0x6f, 0x20, 0x2a, 0x2a, 0x6e, 0x6f, 0x74, 0x2a, 0x2a, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20,
    0x63, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65, 0x72, 0x20, 0x61, 0x73, 0x20, 0x69, 0x74, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x64, 0x0a,
    0x20, 0x6d, 0x61, 0x6e, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x60, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2d, 0x73, 0x69, 0x6e, 0x6b,
    0x2d, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x60, 0x20, 0x74, 0x6f, 0x6f, 0x6c, 0x2e, 0x0a, 0x0a, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20,
    0x75, 0x73, 0x65, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x43, 0x53, 0x56, 0x20, 0x6f, 0x72, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x4c, 0x20,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20,
    0x6c, 0x69, 0x6e, 0x65, 0x2e, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x2c, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x63, 0x61, 0x6e, 0x0a, 0x20, 0x65, 0x78, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c,
    0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x20, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x61, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20, 0x60, 0x73, 0x75, 0x62, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x2d, 0x73, 0x69, 0x6e, 0x6b, 0x2d, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x60, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x6d, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x62, 0x75, 0x6e, 0x64,
    0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x4e, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x18, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x18, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18,
    0x1c, 0x1d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
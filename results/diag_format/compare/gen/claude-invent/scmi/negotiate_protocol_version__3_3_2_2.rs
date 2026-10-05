pub open spec fn negotiate_protocol_version__3_3_2_2_spec(version: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (!IsProtocolVersionSupported(old_s, 0x11u32, version) ==> (
        status == NOT_SUPPORTED
        && NegotiatedProtocolVersion(new_s, 0x11u32) == NegotiatedProtocolVersion(old_s, 0x11u32)
    ))
    && (IsProtocolVersionSupported(old_s, 0x11u32, version) ==> (
        status == SUCCESS
        && NegotiatedProtocolVersion(new_s, 0x11u32) == version
    ))
}

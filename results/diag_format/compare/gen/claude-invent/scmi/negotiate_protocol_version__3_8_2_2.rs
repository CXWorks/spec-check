pub open spec fn negotiate_protocol_version__3_8_2_2_spec(version: u32, result: i32, old_s: S, new_s: S) -> bool {
    (!IsProtocolVersionSupported(old_s, version) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && (IsProtocolVersionSupported(old_s, version) ==> (result == SUCCESS && NegotiatedProtocolVersion(new_s) == version))
}

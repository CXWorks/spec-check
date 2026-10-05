pub open spec fn negotiate_protocol_version__3_10_3_2_spec(old_s: S, new_s: S, version: u32, status: i32) -> bool {
    (!IsProtocolVersionSupported(old_s, version) ==> (status == NOT_SUPPORTED && NegotiatedProtocolVersion(new_s) == NegotiatedProtocolVersion(old_s)))
    && (IsProtocolVersionSupported(old_s, version) ==> (status == SUCCESS && NegotiatedProtocolVersion(new_s) == version))
}

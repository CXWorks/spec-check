pub open spec fn negotiate_protocol_version__3_12_4_2_spec(version: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (!IsProtocolVersionSupported(old_s, version) ==> (status == NOT_SUPPORTED && NegotiatedProtocolVersion(new_s) == NegotiatedProtocolVersion(old_s)))
    && (IsProtocolVersionSupported(old_s, version) ==> (status == SUCCESS && NegotiatedProtocolVersion(new_s) == version))
}

pub open spec fn negotiate_protocol_version__3_7_2_2_spec(version: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
    (!IsProtocolVersionSupported(old_s, version) ==> (status == NOT_SUPPORTED && new_s == old_s))
    && (IsProtocolVersionSupported(old_s, version) ==> (status == SUCCESS && NegotiatedProtocolVersion(new_s) == version))
}

pub open spec fn negotiate_protocol_version__3_3_2_2_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsProtocolVersionSupported(version) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> NegotiatedProtocolVersion() == version)
}
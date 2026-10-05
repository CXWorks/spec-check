pub open spec fn negotiate_protocol_version__3_12_4_2_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsPlatformSupportedProtocolVersion(version) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (NegotiatedProtocolVersion() == version))
}
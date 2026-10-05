pub open spec fn negotiate_protocol_version__3_4_2_2_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsPlatformSupportedProtocolVersion(version) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (NegotiatedProtocolVersion(agent) == version))
}
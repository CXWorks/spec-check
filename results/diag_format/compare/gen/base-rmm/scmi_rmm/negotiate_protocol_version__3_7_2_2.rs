pub open spec fn negotiate_protocol_version__3_7_2_2_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!PlatformSupportsProtocolVersion(version) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (ResultEqual(result, SUCCESS) && NegotiatedProtocolVersion == version))
}
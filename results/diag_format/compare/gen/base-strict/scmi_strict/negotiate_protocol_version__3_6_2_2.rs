pub open spec fn negotiate_protocol_version__3_6_2_2_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!PlatformSupportsProtocolVersion(old_s, version) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (NegotiatedProtocolVersion(new_s, agent) == version && AllSubsequentMessagesComplyWithVersion(new_s, agent, version)))
}
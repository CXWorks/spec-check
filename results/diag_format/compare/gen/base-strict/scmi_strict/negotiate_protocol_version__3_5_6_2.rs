pub open spec fn negotiate_protocol_version__3_5_6_2_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsProtocolVersionSupported(version) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (NegotiatedProtocolVersion(new_s) == version && CommandsResponsesNotificationsComplyWithVersion(new_s, version)))
}
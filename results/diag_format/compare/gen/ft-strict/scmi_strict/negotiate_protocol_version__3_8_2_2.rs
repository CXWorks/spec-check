pub open spec fn negotiate_protocol_version__3_8_2_2_spec(version: UInt32, status: Int32, negotiated_version: NegotiatedProtocolVersion(agent), old_s: S, new_s: S) -> bool {
  (!PlatformSupportsProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s, agent) == version)
  && (result == SUCCESS ==> MessagesComplyWithProtocolVersion(new_s, agent, version))
  && ((PlatformSupportsProtocolVersion(old_s, version))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> NegotiatedProtocolVersion(new_s, agent) == NegotiatedProtocolVersion(old_s, agent))
  && (result != SUCCESS
    ==> MessagesComplyWithProtocolVersion(new_s, agent, version))
}
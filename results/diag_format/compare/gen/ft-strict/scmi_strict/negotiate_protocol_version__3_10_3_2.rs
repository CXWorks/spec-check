pub open spec fn negotiate_protocol_version__3_10_3_2_spec(protocol_id: UInt32, version: UInt32, status: Int32, neg_version: UInt32, old_s: S, new_s: S) -> bool {
  (!PlatformSupportsProtocolVersion(old_s, protocol_id, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> AgentNegotiatedProtocolVersion(new_s, protocol_id) == version)
  && ((PlatformSupportsProtocolVersion(old_s, protocol_id, version))
    ==> ResultEqual(status, SUCCESS))
}
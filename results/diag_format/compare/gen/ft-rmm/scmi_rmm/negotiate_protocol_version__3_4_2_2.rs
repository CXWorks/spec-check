pub open spec fn negotiate_protocol_version__3_4_2_2_spec(version: UInt32, status: Int32, neg_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!IsPlatformSupportedProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s, agent) == version)
  && ((!(IsPlatformSupportedProtocolVersion(old_s, version)))
    ==> ResultEqual(status, SUCCESS))
}
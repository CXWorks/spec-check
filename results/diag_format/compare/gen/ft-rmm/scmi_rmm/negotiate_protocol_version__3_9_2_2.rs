pub open spec fn negotiate_protocol_version__3_9_2_2_spec(version: UInt32, status: Int32, version_neg: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!IsSupportedProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s) == version)
  && ((!(IsSupportedProtocolVersion(old_s, version)))
    ==> ResultEqual(status, SUCCESS))
}
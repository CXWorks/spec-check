pub open spec fn negotiate_protocol_version__3_10_3_2_spec(version: UInt32, status: Int32, neg_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!IsSupportedProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> status == SUCCESS)
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s) == version)
  && ((!(IsSupportedProtocolVersion(old_s, version)))
    ==> NegotiatedProtocolVersion(new_s) == NegotiatedProtocolVersion(old_s))
}
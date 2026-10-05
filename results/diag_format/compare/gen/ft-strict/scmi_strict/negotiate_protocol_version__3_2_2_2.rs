pub open spec fn negotiate_protocol_version__3_2_2_2_spec(version: uint32, status: int32, neg_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!IsPlatformSupportedProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> status == SUCCESS)
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s) == version)
  && ((!(IsPlatformSupportedProtocolVersion(old_s, version)))
    ==> NegotiatedProtocolVersion(new_s) == NegotiatedProtocolVersion(old_s))
}
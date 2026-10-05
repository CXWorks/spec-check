pub open spec fn negotiate_protocol_version__3_6_2_2_spec(version: uint32, status: int32, negotiated_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!IsSupportedProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> status == SUCCESS)
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s) == version)
  && ((!(IsSupportedProtocolVersion(old_s, version)))
    ==> NegotiatedProtocolVersion(new_s) == NegotiatedProtocolVersion(old_s))
}
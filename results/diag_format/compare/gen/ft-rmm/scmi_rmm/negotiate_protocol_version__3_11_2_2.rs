pub open spec fn negotiate_protocol_version__3_11_2_2_spec(version: UInt32, status: Int32, negotiated_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!IsProtocolVersionSupportedByPlatform(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s) == version)
  && (result == SUCCESS ==> status == SUCCESS)
  && ((!(IsProtocolVersionSupportedByPlatform(old_s, version)))
    ==> NegotiatedProtocolVersion(new_s) == NegotiatedProtocolVersion(old_s))
}
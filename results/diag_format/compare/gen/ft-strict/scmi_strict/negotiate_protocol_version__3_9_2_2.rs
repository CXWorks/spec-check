pub open spec fn negotiate_protocol_version__3_9_2_2_spec(version: UInt32, status: Int32, negotiated_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!PlatformSupportsProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result == SUCCESS ==> status == SUCCESS)
  && (result == SUCCESS ==> NegotiatedProtocolVersion(new_s) == version)
  && ((PlatformSupportsProtocolVersion(old_s, version))
    ==> ResultEqual(status, SUCCESS))
}
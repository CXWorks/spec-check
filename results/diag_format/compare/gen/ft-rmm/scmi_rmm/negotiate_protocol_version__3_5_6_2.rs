pub open spec fn negotiate_protocol_version__3_5_6_2_spec(version: UInt32, status: Int32, negotiated_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!PlatformSupportsProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result ==> ResultEqual(status, SUCCESS))
  && (result ==> NegotiatedProtocolVersion(new_s) == version)
  && ((PlatformSupportsProtocolVersion(old_s, version))
    ==> ResultEqual(status, SUCCESS))
}
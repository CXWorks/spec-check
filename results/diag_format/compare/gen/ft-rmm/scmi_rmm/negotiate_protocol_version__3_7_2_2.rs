pub open spec fn negotiate_protocol_version__3_7_2_2_spec(version: UInt32, status: Int32, neg_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!PlatformSupportsProtocolVersion(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result ==> ResultEqual(status, SUCCESS))
  && (result ==> NegotiatedProtocolVersion == version)
  && ((PlatformSupportsProtocolVersion(old_s, version))
    ==> ResultEqual(status, SUCCESS))
}
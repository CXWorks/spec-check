pub open spec fn negotiate_protocol_version__3_8_2_2_spec(version: UInt32, result: Result<(), RmiStatusCode>, negotiated_version: NegotiatedProtocolVersion, old_s: S, new_s: S) -> bool {
  (!PlatformSupportsProtocolVersion(old_s, version) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> NegotiatedProtocolVersion(new_s) == version)
  && ((PlatformSupportsProtocolVersion(old_s, version))
    ==> result.is_Ok())
}
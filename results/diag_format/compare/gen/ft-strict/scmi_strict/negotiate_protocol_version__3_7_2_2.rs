pub open spec fn negotiate_protocol_version__3_7_2_2_spec(version: UInt32, result: Result<(), RmiStatusCode>, version_neg: int, old_s: S, new_s: S) -> bool {
  (!IsSupportedProtocolVersion(old_s, version) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> NegotiatedProtocolVersion(new_s) == version)
  && ((IsSupportedProtocolVersion(old_s, version))
    ==> result.is_Ok())
}
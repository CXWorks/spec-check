pub open spec fn negotiate_protocol_version__3_11_2_2_spec(version: UInt32, status: Int32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsProtocolVersionSupported(old_s, version) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result.is_Ok() ==> ResultEqual(status, SUCCESS))
  && (result.is_Ok() ==> NegotiatedProtocolVersion(new_s) == version)
  && (result.is_Ok() ==> SubsequentMessagesComplyWithVersion(new_s, version))
  && ((IsProtocolVersionSupported(old_s, version))
    ==> ResultEqual(status, SUCCESS))
}
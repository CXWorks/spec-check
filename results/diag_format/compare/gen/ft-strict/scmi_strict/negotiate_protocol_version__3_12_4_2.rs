pub open spec fn negotiate_protocol_version__3_12_4_2_spec(version: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsProtocolVersionSupported(old_s, version) ==> ResultEqual(status, NOT_SUPPORTED))
  && (result ==> ResultEqual(status, SUCCESS))
  && (result ==> NegotiatedProtocolVersion(new_s) == version)
  && (result ==> SubsequentMessagesComplyWithVersion(new_s, version))
  && ((IsProtocolVersionSupported(old_s, version))
    ==> ResultEqual(status, SUCCESS))
}
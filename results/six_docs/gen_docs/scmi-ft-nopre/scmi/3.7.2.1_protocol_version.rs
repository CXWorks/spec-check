pub open spec fn 3.7.2.1_protocol_version_spec(result: RsiCommandReturnCode, version: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> version == 0x30001)
}
pub open spec fn protocol_version__3_8_2_1_spec(status: int32, version: uint32, old_s: S, new_s: S) -> bool {
  (IsSuccessStatus(status))
  && (version == 0x30001)
  && (ProtocolVersionIs(version, 3, 1))
  && ((!(IsSuccessStatus(status)))
    ==> (version == 0x30001))
}
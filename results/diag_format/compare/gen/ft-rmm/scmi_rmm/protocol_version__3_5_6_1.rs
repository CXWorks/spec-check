pub open spec fn protocol_version__3_5_6_1_spec(status: int32, version: uint32, old_s: S, new_s: S) -> bool {
  (StatusIndicatesSuccess(status) && version == 0x40001)
}
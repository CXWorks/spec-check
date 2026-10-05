pub open spec fn protocol_version__3_6_2_1_spec(status: int32, version: uint32, old_s: S, new_s: S) -> bool {
  (IsSuccessStatus(status) ==> version == 0x30000)
}
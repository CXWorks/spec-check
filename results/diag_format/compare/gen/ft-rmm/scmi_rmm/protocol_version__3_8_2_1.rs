pub open spec fn protocol_version__3_8_2_1_spec(status: Int32, version: UInt32, old_s: S, new_s: S) -> bool {
  (status == 0 && version == 0x30001)
}
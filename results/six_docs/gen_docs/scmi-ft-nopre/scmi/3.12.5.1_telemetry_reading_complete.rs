pub open spec fn 3.12.5.1_telemetry_reading_complete_spec(num_dwords: UInt32, array: [UInt32; 1], result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> (num_dwords % 2 == 0))
}
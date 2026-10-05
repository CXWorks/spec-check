pub open spec fn ffa_error_spec(target_info: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  ErrorDeliveredToVcpu(Bits(target_info, 31, 16), Bits(target_info, 15, 0), error_code)
}
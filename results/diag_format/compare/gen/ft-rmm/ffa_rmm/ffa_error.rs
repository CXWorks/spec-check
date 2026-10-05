pub open spec fn ffa_error_spec(error_code: Int32, target_id: UInt16, target_vcpu: UInt16, old_s: S, new_s: S) -> bool {
  ErrorCodeReturnedToPreviousCaller(new_s, error_code)
  && (IsNonSecureVirtualInstance(old_s) && ConduitIs(old_s, SMC) ==> ErrorCodeDeliveredTo(new_s, target_id, target_vcpu, error_code))
  && ((!(IsNonSecureVirtualInstance(old_s)) || !(ConduitIs(old_s, SMC))) ==> ErrorCodeDeliveredTo(new_s, 0, 0, 0))
}
pub open spec fn ffa_error_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> ErrorCodeReturnedToPreviousCaller(error_code))
    && (IsNonSecureVirtualInstance(old_s) && ConduitIs(old_s, SMC) ==> ErrorCodeDeliveredTo(target_id, target_vcpu, error_code))
}
pub open spec fn sbi_hart_suspend_spec(suspend_type: UInt32, resume_addr: UInt64, opaque: UInt64, ret_error: i64, ret_value: i64, old_s: S, new_s: S) -> bool {
    (IsRetentiveSuspendType(suspend_type) ==> (SbiRetIsSuccess(ret_error) && HartRegistersAndCsrsPreserved(old_s, new_s)))
}

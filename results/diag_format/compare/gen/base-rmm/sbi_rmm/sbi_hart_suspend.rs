pub open spec fn sbi_hart_suspend_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (HartIsSuspended(new_s, CallingHart(), old_s.suspend_type) ==> result.ret == 0)
    && (HartResumesOn(new_s, CallingHart(), old_s.suspend_type) ==> result.ret == 0)
    && (IsRetentiveSuspendType(old_s.suspend_type) ==> (result.ret == 0 && HartRegistersAndCsrsPreserved(new_s, CallingHart(), AllPrivilegeModes)))
    && (!IsRetentiveSuspendType(old_s.suspend_type) ==> (result.ret == 0 && !HartRegistersAndCsrsPreserved(new_s, CallingHart(), AllPrivilegeModes)))
}
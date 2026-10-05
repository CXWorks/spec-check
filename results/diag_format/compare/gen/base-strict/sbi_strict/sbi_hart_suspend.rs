pub open spec fn sbi_hart_suspend_spec(result: sbiret, old_s: S, new_s: S, suspend_type: u32, resume_addr: u64, opaque: u64) -> bool {
    (HartEnteredSuspendState(CallingHart(), suspend_type) && HartResumesOnInterruptOrPlatformEvent(CallingHart()))
    && (IsRetentiveSuspendType(suspend_type) ==> (HartRegistersAndCsrsPreserved(CallingHart()) && CallReturnsWithoutFailure(result) && ResumeAddrUnused(resume_addr)))
    && (!IsRetentiveSuspendType(suspend_type) ==> !HartRegistersAndCsrsPreserved(CallingHart()))
}
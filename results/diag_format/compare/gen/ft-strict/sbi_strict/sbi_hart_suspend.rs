pub open spec fn sbi_hart_suspend_spec(suspend_type: uint32_t, resume_addr: unsigned long, opaque: unsigned long, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (HartEnteredSuspendState(new_s, CallingHart(), suspend_type))
  && (HartResumesOnInterruptOrPlatformEvent(new_s, CallingHart()))
  && (IsRetentiveSuspendType(new_s, suspend_type) ==> HartRegistersAndCsrsPreserved(new_s, CallingHart()))
  && (IsRetentiveSuspendType(new_s, suspend_type) ==> CallReturnsWithoutFailure(new_s, result))
  && (IsRetentiveSuspendType(new_s, suspend_type) ==> ResumeAddrUnused(new_s, resume_addr))
  && (!IsRetentiveSuspendType(new_s, suspend_type) ==> !HartRegistersAndCsrsPreserved(new_s, CallingHart()))
  && ((!(HartEnteredSuspendState(old_s, CallingHart(), suspend_type)) &&
       !(HartResumesOnInterruptOrPlatformEvent(old_s, CallingHart())) &&
       !(IsRetentiveSuspendType(old_s, suspend_type) ==> HartRegistersAndCsrsPreserved(old_s, CallingHart())) &&
       !(IsRetentiveSuspendType(old_s, suspend_type) ==> CallReturnsWithoutFailure(old_s, result)) &&
       !(IsRetentiveSuspendType(old_s, suspend_type) ==> ResumeAddrUnused(old_s, resume_addr)) &&
       !(!IsRetentiveSuspendType(old_s, suspend_type) ==> !HartRegistersAndCsrsPreserved(old_s, CallingHart())))
    ==> result.code == 0)
}
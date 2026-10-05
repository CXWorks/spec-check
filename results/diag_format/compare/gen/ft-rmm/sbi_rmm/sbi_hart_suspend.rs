pub open spec fn sbi_hart_suspend_spec(suspend_type: UInt32, resume_addr: unsigned long, opaque: unsigned long, ret: struct sbiret, old_s: S, new_s: S) -> bool {
  (HartIsSuspended(new_s, CallingHart(), suspend_type))
  && (HartResumesOn(new_s, CallingHart(), InterruptOrPlatformEvent()))
  && (IsRetentiveSuspendType(old_s, suspend_type) ==> CallReturnsWithoutFailure(new_s, ret))
  && (IsRetentiveSuspendType(old_s, suspend_type) ==> HartRegistersAndCsrsPreserved(new_s, CallingHart(), AllPrivilegeModes))
  && (!IsRetentiveSuspendType(old_s, suspend_type) ==> !HartRegistersAndCsrsPreserved(new_s, CallingHart(), AllPrivilegeModes))
  && ((!HartIsSuspended(new_s, CallingHart(), suspend_type))
    ==> CallReturnsWithFailure(ret))
}
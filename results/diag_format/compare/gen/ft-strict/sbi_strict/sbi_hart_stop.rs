pub open spec fn sbi_hart_stop_spec(calling_hart: CallingHart, old_s: S, new_s: S) -> bool {
  (!HartExecutingInSupervisorMode(old_s, calling_hart) ==> !HartExecutingInSupervisorMode(new_s, calling_hart))
  && (HartOwnedBySbiImplementation(old_s, calling_hart) ==> HartOwnedBySbiImplementation(new_s, calling_hart))
  && (CallReturnsToCaller(old_s, calling_hart) ==> CallReturnsToCaller(new_s, calling_hart))
  && ((HartExecutingInSupervisorMode(old_s, calling_hart))
    ==> HartExecutingInSupervisorMode(new_s, calling_hart))
  && ((!HartOwnedBySbiImplementation(old_s, calling_hart))
    ==> HartOwnedBySbiImplementation(new_s, calling_hart))
  && ((!CallReturnsToCaller(old_s, calling_hart))
    ==> CallReturnsToCaller(new_s, calling_hart))
}
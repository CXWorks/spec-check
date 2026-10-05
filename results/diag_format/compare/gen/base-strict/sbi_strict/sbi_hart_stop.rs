pub open spec fn sbi_hart_stop_spec(result: Int, value: Int, old_s: S, new_s: S) -> bool {
    (!HartExecutingInSupervisorMode(old_s, calling_hart) ==> result != 0)
    && (HartExecutingInSupervisorMode(old_s, calling_hart) ==> (result == 0 && !HartExecutingInSupervisorMode(new_s, calling_hart) && HartOwnedBySbiImplementation(new_s, calling_hart) && !CallReturnsToCaller(new_s, calling_hart)))
}
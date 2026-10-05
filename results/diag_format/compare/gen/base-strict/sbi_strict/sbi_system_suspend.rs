pub open spec fn sbi_system_suspend_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    // Failure condition: if the call returns, it failed (result != 0)
    (result != 0 ==> true)
    // Success conditions: if the call does not return (result == 0), then the postconditions hold
    // Note: Since the command does not return on success, we model the postconditions as constraints on the hypothetical new state
    // that would exist if the command were to complete successfully.
    (result == 0 ==>
        HartResumesFromState(old_s, CallingHart(), STOPPED)
        && HartPrivilegeMode(old_s, CallingHart()) == SUPERVISOR
        && HartResumesAtAddress(old_s, CallingHart(), resume_addr)
        && satp == 0
        && sstatus.SIE == 0
        && a0 == HartId(CallingHart())
        && a1 == opaque
        && OtherRegistersUndefined(CallingHart())
    )
}
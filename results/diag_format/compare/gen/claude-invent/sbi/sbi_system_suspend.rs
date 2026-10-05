pub open spec fn sbi_system_suspend_spec(sleep_type: u32, resume_addr: u64, opaque: u64, returned: bool, ret_error: i64, old_s: S, new_s: S) -> bool {
    (returned ==> ret_error != 0)
    && (!returned ==> (
        HartResumedFromStopped(old_s, new_s, CurrentHartId(old_s))
        && CurrentHartId(new_s) == CurrentHartId(old_s)
        && IsSupervisorMode(new_s)
        && HartPc(new_s) == resume_addr
        && HartSatp(new_s) == 0
        && HartSstatusSie(new_s) == 0
        && HartRegA0(new_s) == CurrentHartId(old_s)
        && HartRegA1(new_s) == opaque
    ))
}

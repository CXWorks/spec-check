pub open spec fn sbi_system_suspend_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error != 0 ==> true)
    && (result.error == 0 ==> (old_s.hart_state == HART_STATE_STOPPED && new_s.hart_state == HART_STATE_RUNNING))
}
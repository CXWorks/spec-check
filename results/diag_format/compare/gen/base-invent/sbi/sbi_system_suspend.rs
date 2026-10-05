pub open spec fn sbi_system_suspend_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error != 0 ==> true)
    && (result.error == 0 ==> (new_s.hart_state == HART_STOPPED && (new_s.a0 as int) == old_s.hartid && (new_s.a1 as int) == old_s.opaque))
}
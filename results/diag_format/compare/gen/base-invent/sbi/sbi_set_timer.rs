pub open spec fn sbi_set_timer_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS)
    && (new_s.sie_stie == old_s.sie_stie)
    && (new_s.sie_timer_pending == false)
}
pub open spec fn sbi_set_timer_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_SUCCESS)
    && (new_s.sie_stie == old_s.sie_stie)
}
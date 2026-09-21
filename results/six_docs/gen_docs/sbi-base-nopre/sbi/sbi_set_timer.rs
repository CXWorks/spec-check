pub open spec fn sbi_set_timer_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS)
    && (result.value == ())
    && (old_s == new_s)
}
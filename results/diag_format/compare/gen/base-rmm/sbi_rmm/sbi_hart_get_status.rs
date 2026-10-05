pub open spec fn sbi_hart_get_status_spec(error: i64, value: i64, hartid: u64, old_s: S, new_s: S) -> bool {
    (!IsValidHartid(hartid) ==> error == SBI_ERR_INVALID_PARAM)
    && (IsHsmStateId(value))
    && (old_s == new_s)
}
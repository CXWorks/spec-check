pub open spec fn sbi_hart_get_status_spec(result: int, hartid: UInt64, old_s: S, new_s: S) -> bool {
    (hartid as int < 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (hartid as int >= 0 ==> true)
}
pub open spec fn sbi_hart_get_status_spec(result: int, hartid: UInt64, old_s: S, new_s: S) -> bool {
    (hartid as int < 0 ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (hartid as int >= 0 && hartid as int >= (old_s.hart_count as int) ==> ResultEqual(result, SBI_SBI_ERR_INVALID_PARAM))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> old_s.hart_count as int <= hartid as int)
    && (result != SBI_SBI_ERR_INVALID_PARAM ==> old_s.hart_count as int > hartid as int)
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> new_s.hart_count == old_s.hart_count)
    && (result != SBI_SBI_ERR_INVALID_PARAM ==> new_s.hart_count == old_s.hart_count)
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> new_s.hart_states == old_s.hart_states)
    && (result != SBI_SBI_ERR_INVALID_PARAM ==> new_s.hart_states == old_s.hart_states)
}
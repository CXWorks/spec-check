pub open spec fn sbi_sse_register_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_STATE ==> old_s.event_state(old_s.event_id) != UNUSED)
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.event_id() as int) < 0 || (old_s.handler_entry_pc() as int) % 2 != 0)
    && (result == SBI_SBI_ERR_NOT_SUPPORTED ==> true)
    && (result == SBI_SBI_SUCCESS ==> old_s.event_state(old_s.event_id) == UNUSED && new_s.event_state(old_s.event_id) == REGISTERED)
    && (result != SBI_SBI_SUCCESS && result != SBI_SBI_ERR_INVALID_STATE && result != SBI_SBI_ERR_INVALID_PARAM && result != SBI_SBI_ERR_NOT_SUPPORTED ==> true)
}
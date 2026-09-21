pub open spec fn sbi_sse_register_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result == SBI_ERR_INVALID_STATE ==> !EventIsUnused(old_s, event_id))
    && (result == SBI_ERR_INVALID_PARAM ==> (event_id as int < 0 || (handler_entry_pc as int) % 2 != 0))
    && (result == SBI_ERR_NOT_SUPPORTED ==> true)
    && (result == SBI_SUCCESS ==> EventIsUnused(old_s, event_id) && EventIsRegistered(new_s, event_id))
}
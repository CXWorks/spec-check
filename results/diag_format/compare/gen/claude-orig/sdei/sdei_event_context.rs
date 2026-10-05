pub open spec fn sdei_event_context_spec(param_id: u32, result: i64, old_s: S, new_s: S) -> bool {
    ((param_id as int) > 17 ==> (result == INVALID_PARAMETERS || result == DENIED))
    && (new_s == old_s)
}

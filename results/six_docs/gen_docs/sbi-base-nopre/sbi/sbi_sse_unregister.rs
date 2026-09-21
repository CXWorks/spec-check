pub open spec fn sbi_sse_unregister_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (event_is_registered(old_s, result.event_id) ==> result.error != 0)
    && (result.error == 0 ==> event_state_is_unused(new_s, result.event_id))
    && (result.error == 0 ==> event_state_is_not_registered(old_s, result.event_id))
}
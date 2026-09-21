pub open spec fn sbi_sse_disable_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (event_enabled(old_s, event_id) ==> result.code() == SBI_SUCCESS)
    && (!event_enabled(old_s, event_id) ==> result.code() != SBI_SUCCESS)
    && (result.code() == SBI_SUCCESS ==> event_registered(new_s, event_id))
    && (result.code() == SBI_SUCCESS ==> !event_enabled(new_s, event_id))
}
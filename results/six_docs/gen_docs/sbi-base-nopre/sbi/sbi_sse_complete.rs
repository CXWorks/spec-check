pub open spec fn sbi_sse_complete_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (old_s.hart_events_running_count() == 0 ==> result == SBI_SUCCESS)
    && (old_s.hart_events_running_count() > 0 ==> {
        let highest_event = old_s.hart_events_running_highest_priority();
        let is_one_shot = old_s.event_configured_as_one_shot(highest_event);
        let new_state = if is_one_shot { REGISTERED } else { ENABLED };
        new_s.hart_events_running_count() == old_s.hart_events_running_count() - 1
        && new_s.hart_events_registered_count() == old_s.hart_events_registered_count() + (if is_one_shot { 1 } else { 0 })
        && new_s.hart_events_enabled_count() == old_s.hart_events_enabled_count() + (if !is_one_shot { 1 } else { 0 })
        && new_s.hart_event_state(highest_event) == new_state
        && new_s.hart_events_running_highest_priority() == old_s.hart_events_running_next_priority()
        && new_s.hart_state() == old_s.hart_state()
    })
}
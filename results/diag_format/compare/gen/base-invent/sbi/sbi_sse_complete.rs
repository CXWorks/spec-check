pub open spec fn sbi_sse_complete_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let events = &old_s.sse_events;
    let calling_hart = old_s.hart_id;
    let mut highest_priority_event = None;
    let mut highest_priority = u64::MAX;
    for (i, event) in events.iter().enumerate() {
        if event.hart_id == calling_hart && event.state == SseEventState::RUNNING {
            if event.priority < highest_priority {
                highest_priority = event.priority;
                highest_priority_event = Some(i);
            }
        }
    }
    let event_index = highest_priority_event;
    (event_index.is_none() ==> result == SBI_SUCCESS)
    && (event_index.is_some() ==> {
        let idx = event_index.unwrap();
        let event = &events[idx];
        let event_is_one_shot = event.config == SseEventConfig::ONE_SHOT;
        let new_state = if event_is_one_shot {
            SseEventState::REGISTERED
        } else {
            SseEventState::ENABLED
        };
        result == SBI_SUCCESS
            && new_s.hart_id == old_s.hart_id
            && new_s.sse_events == old_s.sse_events
            && new_s.sse_events[idx].state == new_state
            && new_s.sse_events[idx].priority == event.priority
            && new_s.sse_events[idx].config == event.config
    })
}
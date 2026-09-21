pub open spec fn sbi_sse_complete_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_SUCCESS)
    && (forall *e: Event where e.state == RUNNING && e.hart == old_s.hart ==> {
        new_s.hart_events[e.hart] == old_s.hart_events[e.hart]
    })
    && (forall *e: Event where e.state == RUNNING && e.hart == old_s.hart && e.priority == old_s.hart_events[old_s.hart].max_priority ==> {
        new_s.hart_events[e.hart] == old_s.hart_events[e.hart]
        && (e.config == ONE_SHOT ==> new_s.hart_events[e.hart].max_priority < e.priority || new_s.hart_events[e.hart].max_priority == -1)
        && (e.config != ONE_SHOT ==> new_s.hart_events[e.hart].max_priority == e.priority)
    })
    && (forall *e: Event where e.state != RUNNING || e.hart != old_s.hart ==> {
        new_s.hart_events[e.hart] == old_s.hart_events[e.hart]
    })
}
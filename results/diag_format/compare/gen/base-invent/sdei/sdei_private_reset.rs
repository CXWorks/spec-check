pub open spec fn sdei_private_reset_spec(result: i64, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (all_private_events_unregistered(old_s, new_s) && no_running_handlers(old_s, new_s)))
    && (result == SDEI_NOT_SUPPORTED ==> sdei_not_supported(old_s))
    && (result == SDEI_DENIED ==> (exists_running_handler(old_s)))
}

pub open spec fn all_private_events_unregistered(old_s: S, new_s: S) -> bool {
    forall (e: EventId) ::
        old_s.private_events.contains(e) ==> new_s.private_events.contains(e) == false
}

pub open spec fn no_running_handlers(old_s: S, new_s: S) -> bool {
    forall (e: EventId) ::
        old_s.private_events.contains(e) && old_s.event_handlers[e].running == true ==>
            new_s.event_handlers[e].running == false
}

pub open spec fn sdei_not_supported(old_s: S) -> bool {
    old_s.sdei_supported == false
}

pub open spec fn exists_running_handler(old_s: S) -> bool {
    exists (e: EventId) ::
        old_s.private_events.contains(e) && old_s.event_handlers[e].running == true
}
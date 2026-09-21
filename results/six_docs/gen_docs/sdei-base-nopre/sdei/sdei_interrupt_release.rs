pub open spec fn sdei_interrupt_release_spec(result: i32, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (event_is_bound(old_s, event) && event_handler_is_unregistered(old_s, event) && event_is_not_bound(new_s, event)))
    && (result == SDEI_NOT_SUPPORTED ==> !sdei_supported(old_s))
    && (result == SDEI_INVALID_PARAMETERS ==> (!event_is_bound(old_s, event) || event_is_invalid(old_s, event)))
    && (result == SDEI_DENIED ==> !event_handler_is_unregistered(old_s, event))
}
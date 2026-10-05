pub open spec fn sdei_event_routing_set_spec(result: int, old_s: S, new_s: S, event: i32, routing_mode: u64, affinity: u64) -> bool {
    (!SdeiEventIsValid(old_s, event) ==> result == INVALID_PARAMETERS)
    && ((SdeiEventIsValid(old_s, event) && !SdeiEventIsShared(old_s, event)) ==> result == INVALID_PARAMETERS)
    && ((routing_mode >> 1u64) != 0u64 ==> result == INVALID_PARAMETERS)
    && (((routing_mode & 1u64) == 1u64 && !SdeiAffinityIsValid(old_s, affinity)) ==> result == INVALID_PARAMETERS)
    && ((SdeiEventIsValid(old_s, event)
        && SdeiEventIsShared(old_s, event)
        && (routing_mode >> 1u64) == 0u64
        && ((routing_mode & 1u64) == 0u64 || SdeiAffinityIsValid(old_s, affinity))
        && SdeiEventHandlerState(old_s, event) != HANDLER_REGISTERED) ==> result == DENIED)
    && ((result != SUCCESS) ==> new_s == old_s)
    && ((SdeiEventIsValid(old_s, event)
        && SdeiEventIsShared(old_s, event)
        && (routing_mode >> 1u64) == 0u64
        && ((routing_mode & 1u64) == 0u64 || SdeiAffinityIsValid(old_s, affinity))
        && SdeiEventHandlerState(old_s, event) == HANDLER_REGISTERED) ==>
        (result == SUCCESS
        && SdeiEventRoutingMode(new_s, event) == (routing_mode & 1u64)
        && ((routing_mode & 1u64) == 1u64 ==> SdeiEventAffinity(new_s, event) == affinity)
        && SdeiEventHandlerState(new_s, event) == SdeiEventHandlerState(old_s, event)))
}

pub open spec fn sdei_event_routing_set_spec(event: i32, routing_mode: u64, affinity: u64, result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s)
        && (!SdeiEventIsValid(old_s, event)
            || !SdeiEventIsShared(old_s, event)
            || (routing_mode >> 1u64) != 0u64
            || ((routing_mode & 1u64) == 1u64 && !SdeiAffinityIsValid(old_s, affinity))))
        ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((SdeiIsSupported(old_s)
        && SdeiEventIsValid(old_s, event)
        && SdeiEventIsShared(old_s, event)
        && (routing_mode >> 1u64) == 0u64
        && ((routing_mode & 1u64) == 1u64 ==> SdeiAffinityIsValid(old_s, affinity))
        && !SdeiEventHandlerIsRegistered(old_s, event))
        ==> (result == DENIED && new_s == old_s))
    && ((SdeiIsSupported(old_s)
        && SdeiEventIsValid(old_s, event)
        && SdeiEventIsShared(old_s, event)
        && (routing_mode >> 1u64) == 0u64
        && ((routing_mode & 1u64) == 1u64 ==> SdeiAffinityIsValid(old_s, affinity))
        && SdeiEventHandlerIsRegistered(old_s, event))
        ==> (result == SUCCESS
            && SdeiEventRoutingMode(new_s, event) == (routing_mode & 1u64)
            && ((routing_mode & 1u64) == 1u64 ==> SdeiEventAffinity(new_s, event) == affinity)
            && SdeiEventHandlerIsRegistered(new_s, event)
            && SdeiEventIsShared(new_s, event)))
}

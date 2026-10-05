pub open spec fn sdei_event_register_spec(result: i64, old_s: S, new_s: S, event: i32, entry_point_address: UInt64, ep_argument: UInt64, flags: UInt64, affinity: UInt64) -> bool {
    let routing_mode: UInt64 = flags & 1u64;
    let relative_mode: UInt64 = (flags >> 1u64) & 1u64;
    let not_supported: bool = !SdeiIsSupported(old_s);
    let invalid_parameters: bool =
        !SdeiEventIsValid(old_s, event)
        || (flags >> 2u64) != 0
        || !SdeiEntryPointIsValid(old_s, entry_point_address, relative_mode)
        || (SdeiEventIsShared(old_s, event)
            && routing_mode == 1u64
            && (((affinity >> 40u64) != 0)
                || (((affinity >> 24u64) & 0xFFu64) != 0)
                || !SdeiAffinityIsValid(old_s, affinity)));
    let denied: bool =
        SdeiEventIsRegistered(old_s, event)
        || SdeiEventIsUnregisterPending(old_s, event);
    (not_supported ==> result == NOT_SUPPORTED)
    && ((!not_supported && invalid_parameters) ==> result == INVALID_PARAMETERS)
    && ((!not_supported && !invalid_parameters && denied) ==> result == DENIED)
    && ((not_supported || invalid_parameters || denied) ==> new_s == old_s)
    && ((!not_supported && !invalid_parameters && !denied) ==> (
        result == SUCCESS
        && SdeiEventIsRegistered(new_s, event)
        && !SdeiEventIsEnabled(new_s, event)
        && !SdeiEventIsUnregisterPending(new_s, event)
        && SdeiEventHandlerAddress(new_s, event) == entry_point_address
        && SdeiEventHandlerRelativeMode(new_s, event) == relative_mode
        && SdeiEventHandlerArgument(new_s, event) == ep_argument
        && (SdeiEventIsShared(old_s, event) ==> (
            SdeiEventRoutingMode(new_s, event) == routing_mode
            && (routing_mode == 1u64 ==> SdeiEventAffinity(new_s, event) == affinity)))
    ))
}

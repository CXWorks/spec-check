pub open spec fn sdei_event_get_info_spec(event: i32, info: u32, result: i64, old_s: S, new_s: S) -> bool {
    (new_s == old_s)
    && (!SdeiIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((SdeiIsSupported(old_s) && (!SdeiEventIsValid(old_s, event) || info > 4u32))
        ==> result == INVALID_PARAMETERS)
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 3u32
        && !SdeiEventIsShared(old_s, event))
        ==> result == INVALID_PARAMETERS)
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 3u32
        && SdeiEventIsShared(old_s, event)
        && !SdeiEventIsHandlerRegistered(old_s, event))
        ==> result == DENIED)
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 4u32
        && !SdeiEventIsShared(old_s, event))
        ==> result == INVALID_PARAMETERS)
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 4u32
        && SdeiEventIsShared(old_s, event)
        && !SdeiEventIsHandlerRegistered(old_s, event))
        ==> result == DENIED)
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 4u32
        && SdeiEventIsShared(old_s, event)
        && SdeiEventIsHandlerRegistered(old_s, event)
        && SdeiEventRoutingMode(old_s, event) != 1int)
        ==> result == INVALID_PARAMETERS)
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 0u32)
        ==> (result == (if SdeiEventIsShared(old_s, event) { 1i64 } else { 0i64 })))
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 1u32)
        ==> (result == (if SdeiEventCanBeSoftwareSignaled(old_s, event) { 0i64 } else { 1i64 })))
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 2u32)
        ==> (result == (if SdeiEventIsCriticalPriority(old_s, event) { 1i64 } else { 0i64 })))
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 3u32
        && SdeiEventIsShared(old_s, event)
        && SdeiEventIsHandlerRegistered(old_s, event))
        ==> ((result as int) == SdeiEventRoutingMode(old_s, event)
            && (SdeiEventRoutingMode(old_s, event) == 0int || SdeiEventRoutingMode(old_s, event) == 1int)))
    && ((SdeiIsSupported(old_s) && SdeiEventIsValid(old_s, event) && info == 4u32
        && SdeiEventIsShared(old_s, event)
        && SdeiEventIsHandlerRegistered(old_s, event)
        && SdeiEventRoutingMode(old_s, event) == 1int)
        ==> ((result as int) == SdeiEventRoutingAffinity(old_s, event)))
}

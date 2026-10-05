pub open spec fn sdei_private_reset_spec(function_id: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
    (function_id == 0xC400_0031u32)
    && (!SdeiIsSupported(old_s) ==> result == SDEI_NOT_SUPPORTED)
    && ((SdeiIsSupported(old_s) && AnyPrivateEventHandlerRunning(old_s, CallingPe(old_s))) ==> result == SDEI_DENIED)
    && ((SdeiIsSupported(old_s) && !AnyPrivateEventHandlerRunning(old_s, CallingPe(old_s))) ==> (
        result == SDEI_SUCCESS
        && AllPrivateEventsUnregistered(new_s, CallingPe(old_s))
        && PrivateEventAuxInfoWarmBootState(new_s, CallingPe(old_s))
        && SharedEventsUnchanged(old_s, new_s)
    ))
    && ((result == SDEI_NOT_SUPPORTED || result == SDEI_DENIED) ==> SdeiStateUnchanged(old_s, new_s))
    && (result == SDEI_SUCCESS || result == SDEI_NOT_SUPPORTED || result == SDEI_DENIED)
}

pub open spec fn sdei_event_context_spec(param_id: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((param_id as int) > 17 ==> result == INVALID_PARAMETERS)
    && (!SdeiHandlerRunningOnCallingPe(old_s) ==> result == DENIED)
    && ((SdeiIsSupported(old_s)
        && (param_id as int) <= 17
        && SdeiHandlerRunningOnCallingPe(old_s))
        ==> (result == SdeiEventContextRegister(old_s, param_id as int)
            && new_s == old_s))
}

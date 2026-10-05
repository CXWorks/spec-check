pub open spec fn sdei_event_context_spec(param_id: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidContextParamId(param_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsHandlerRunningOnPe(old_s, CurrentPe(old_s)) ==> ResultEqual(result, DENIED))
    && ((IsSdeiSupported(old_s)
        && IsValidContextParamId(param_id)
        && IsHandlerRunningOnPe(old_s, CurrentPe(old_s)))
        ==> (result == EventContextRegister(old_s, CurrentPe(old_s), param_id)))
    && (new_s == old_s)
}

pub open spec fn sdei_event_context_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidContextParamId(param_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsHandlerRunningOnPe(CurrentPe()) ==> ResultEqual(result, DENIED))
    && (IsSdeiSupported() && IsValidContextParamId(param_id) && IsHandlerRunningOnPe(CurrentPe()) ==> result == EventContextRegister(CurrentPe(), param_id))
    && (old_s == new_s)
}
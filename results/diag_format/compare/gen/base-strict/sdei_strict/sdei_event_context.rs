pub open spec fn sdei_event_context_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (param_id > 17 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!HandlerRunning(CallingPe()) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, EventContextRegister(CallingPe(), param_id)) ==> true)
    && (true ==> new_s == old_s)
}
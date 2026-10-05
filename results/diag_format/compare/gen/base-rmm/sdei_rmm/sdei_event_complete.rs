pub open spec fn sdei_event_complete_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!HandlerRunning(CallingPe()) ==> ResultEqual(result, DENIED))
    && (result == NOT_SUPPORTED || result == DENIED || HandlerRunning(CallingPe()))
    && (HandlerRunning(CallingPe()) ==> HandlerRunning(CallingPe()))
    && (IsPrivateEvent(event) ==> EventHandlingComplete(event, CallingPe()))
    && (IsSharedEvent(event) ==> EventHandlingCompleteGlobally(event))
}
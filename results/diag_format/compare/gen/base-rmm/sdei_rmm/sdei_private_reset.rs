pub open spec fn sdei_private_reset_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (AnyEventHandlerRunning(CallingPe()) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        (forall ev in PrivateEvents(CallingPe()): !EventIsRunning(ev) ==> EventState(new_s, ev) == UNREGISTERED)
        && (forall ev in PrivateEvents(CallingPe()): EventIsRunning(ev) ==> EventState(new_s, ev) == HANDLER_UNREGISTER_PENDING)
        && (forall ev in PrivateEvents(CallingPe()): EventAuxInfoIsCleared(new_s, ev) || EventAuxInfoIsWarmBootState(new_s, ev))
        && (forall ev in SharedEvents(): EventState(new_s, ev) == EventState(old_s, ev))
    ))
}
pub open spec fn sdei_private_reset_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (AnyEventHandlerRunning(old_s, CallingPe(old_s)) ==> ResultEqual(result, DENIED))
    && ((SdeiIsSupported(old_s) && !AnyEventHandlerRunning(old_s, CallingPe(old_s))) ==> (
        ResultEqual(result, SUCCESS)
        && (forall |ev: int| PrivateEvents(old_s, CallingPe(old_s)).contains(ev) ==>
                (!EventIsRunning(new_s, ev) ==> EventState(new_s, ev) == UNREGISTERED))
        && (forall |ev: int| PrivateEvents(old_s, CallingPe(old_s)).contains(ev) ==>
                (EventIsRunning(new_s, ev) ==> EventState(new_s, ev) == HANDLER_UNREGISTER_PENDING))
        && (forall |ev: int| PrivateEvents(old_s, CallingPe(old_s)).contains(ev) ==>
                (EventAuxInfoIsCleared(new_s, ev) || EventAuxInfoIsWarmBootState(new_s, ev)))
        && (forall |ev: int| SharedEvents(old_s).contains(ev) ==>
                EventState(new_s, ev) == EventState(old_s, ev))
    ))
}

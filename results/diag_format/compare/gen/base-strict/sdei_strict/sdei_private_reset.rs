pub open spec fn sdei_private_reset_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (exists|e: SdeiEvent| IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && HandlerRunning(e) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        ResultEqual(result, SUCCESS)
        && forall|e: SdeiEvent| (IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && !HandlerRunning(e) ==> EventIsUnregistered(e))
        && forall|e: SdeiEvent| (IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && HandlerRunning(e) ==> HandlerUnregisterPending(e))
        && PrivateEventAuxInfoReset(CallingPe())
        && forall|e: SdeiEvent| IsSharedEvent(e) ==> EventStateUnchanged(e)
    ))
}
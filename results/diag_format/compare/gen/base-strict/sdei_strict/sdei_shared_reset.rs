pub open spec fn sdei_shared_reset_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (exists|e: SdeiEvent| IsSharedEvent(e) && e.handler_running == TRUE ==> ResultEqual(result, DENIED))
    && (exists|e: SdeiEvent| IsInterruptBoundEvent(e) && IsEventRegistered(e) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (forall|e: SdeiEvent| IsSharedEvent(e) ==> !IsEventRegistered(e)))
    && (ResultEqual(result, SUCCESS) ==> (forall|i: Interrupt| !IsInterruptBoundToEvent(i)))
    && (ResultEqual(result, SUCCESS) ==> SharedAuxiliaryInfoCleared())
    && (ResultEqual(result, SUCCESS) ==> (forall|e: SdeiEvent| IsPrivateEvent(e) ==> PrivateEventStateUnchanged(e)))
}
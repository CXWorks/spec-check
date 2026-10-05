pub open spec fn sdei_shared_reset_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (AnySharedEventHandlerRunning() ==> ResultEqual(result, DENIED))
    && (AnyInterruptEventBindingRegistered() ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> !AnySharedEventRegistered())
    && (ResultEqual(result, SUCCESS) ==> !AnyInterruptBoundToEvent())
    && (ResultEqual(result, SUCCESS) ==> SharedEventAuxInfoCleared())
    && (ResultEqual(result, SUCCESS) ==> InterruptBindingAuxInfoCleared())
    && (ResultEqual(result, SUCCESS) ==> PrivateEventState() == old(PrivateEventState()))
}
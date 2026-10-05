pub open spec fn sdei_shared_reset_spec(fid: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((SdeiIsSupported(old_s) && AnySharedEventHandlerRunning(old_s)) ==> ResultEqual(result, DENIED))
    && ((SdeiIsSupported(old_s) && AnyInterruptEventBindingRegistered(old_s)) ==> ResultEqual(result, DENIED))
    && ((SdeiIsSupported(old_s)
        && !AnySharedEventHandlerRunning(old_s)
        && !AnyInterruptEventBindingRegistered(old_s)) ==> (
            ResultEqual(result, SUCCESS)
            && !AnySharedEventRegistered(new_s)
            && !AnyInterruptBoundToEvent(new_s)
            && SharedEventAuxInfoCleared(new_s)
            && InterruptBindingAuxInfoCleared(new_s)
            && PrivateEventState(new_s) == PrivateEventState(old_s)
        ))
}

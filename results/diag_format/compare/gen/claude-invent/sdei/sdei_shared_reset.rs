pub open spec fn sdei_shared_reset_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((SdeiIsSupported(old_s)
         && (SdeiAnySharedEventHandlerRunning(old_s) || SdeiAnyInterruptEventBindingRegistered(old_s)))
        ==> (result == DENIED && new_s == old_s))
    && ((SdeiIsSupported(old_s)
         && !SdeiAnySharedEventHandlerRunning(old_s)
         && !SdeiAnyInterruptEventBindingRegistered(old_s))
        ==> (result == SUCCESS
             && SdeiNoSharedEventRegistered(new_s)
             && !SdeiAnyInterruptEventBindingRegistered(new_s)
             && SdeiSharedEventAuxInfoCleared(new_s)
             && SdeiPrivateEventsUnchanged(old_s, new_s)))
}

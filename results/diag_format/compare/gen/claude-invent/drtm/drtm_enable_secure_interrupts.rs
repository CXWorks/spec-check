pub open spec fn drtm_enable_secure_interrupts_spec(result: i64, old_s: S, new_s: S) -> bool {
    (!DrtmIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((DrtmIsSupported(old_s) && (!DynamicLaunchOccurred(old_s) || !SecureInterruptsDisabled(old_s))) ==> result == DENIED)
    && (result == SUCCESS ==> (DrtmIsSupported(old_s) && DynamicLaunchOccurred(old_s) && SecureInterruptsDisabled(old_s) && !SecureInterruptsDisabled(new_s)))
    && (result != SUCCESS ==> new_s == old_s)
}

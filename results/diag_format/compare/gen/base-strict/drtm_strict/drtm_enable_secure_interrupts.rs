pub open spec fn drtm_enable_secure_interrupts_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsDrtmSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!DynamicLaunchOccurred() ==> ResultEqual(result, DENIED))
    && (!SecureInterruptsDisabled() ==> ResultEqual(result, DENIED))
    && (!SecureInterruptDisableRequestedInDrtmParameters() ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (SecureInterruptsEnabled() || !SecureInterruptsInUseByPlatform()))
    && (SecureInterruptEnableState() == SecureInterruptEnableState())
}
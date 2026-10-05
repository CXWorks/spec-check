pub open spec fn drtm_enable_secure_interrupts_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!HasDynamicLaunchOccurred(old_s) ==> ResultEqual(result, DENIED))
    && (!AreSecureInterruptsDisabled(old_s) ==> ResultEqual(result, DENIED))
    && (!DrtmParametersRequestedSecureInterruptDisable(old_s) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> AreSecureInterruptsEnabled(new_s))
    && (AreSecureInterruptsInUseByPlatform(old_s) ==> AreSecureInterruptsEnabled(new_s))
    && (AreSecureInterruptsEnabled(new_s) ==> SecureInterruptEnableState(new_s))
}
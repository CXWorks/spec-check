pub open spec fn drtm_enable_secure_interrupts_spec(result: Result<(), DrtmStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!HasDynamicLaunchOccurred(old_s) ==> ResultEqual(result, DENIED))
  && (!AreSecureInterruptsDisabled(old_s) ==> ResultEqual(result, DENIED))
  && (!DrtmParametersRequestedSecureInterruptDisable(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS && AreSecureInterruptsInUseByPlatform(old_s) ==> AreSecureInterruptsEnabled(new_s))
  && ((IsDrtmSupported(old_s) &&
       HasDynamicLaunchOccurred(old_s) &&
       AreSecureInterruptsDisabled(old_s) &&
       DrtmParametersRequestedSecureInterruptDisable(old_s))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> AreSecureInterruptsEnabled(new_s) == AreSecureInterruptsEnabled(old_s))
}
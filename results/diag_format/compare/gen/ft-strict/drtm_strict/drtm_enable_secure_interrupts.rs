pub open spec fn drtm_enable_secure_interrupts_spec(result: Result<(), DrtmCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsDrtmSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!DynamicLaunchOccurred(old_s) ==> ResultEqual(result, DENIED))
  && (!SecureInterruptsDisabled(old_s) ==> ResultEqual(result, DENIED))
  && (!SecureInterruptDisableRequestedInDrtmParameters(old_s) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> SecureInterruptsEnabled(new_s) || !SecureInterruptsInUseByPlatform(new_s))
  && ((IsDrtmSupported(old_s) &&
       DynamicLaunchOccurred(old_s) &&
       SecureInterruptsDisabled(old_s) &&
       SecureInterruptDisableRequestedInDrtmParameters(old_s))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> SecureInterruptsEnabled(new_s) == SecureInterruptsEnabled(old_s))
}
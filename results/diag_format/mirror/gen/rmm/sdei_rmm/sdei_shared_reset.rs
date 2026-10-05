pub open spec fn sdei_shared_reset_spec(result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (AnySharedEventHandlerRunning(old_s) ==> ResultEqual(result, DENIED))
  && (AnyInterruptEventBindingRegistered(old_s) ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS ==> !AnySharedEventRegistered(new_s))
  && (result == SDEI_SUCCESS ==> !AnyInterruptBoundToEvent(new_s))
  && (result == SDEI_SUCCESS ==> SharedEventAuxInfoCleared(new_s) && InterruptBindingAuxInfoCleared(new_s))
  && (result == SDEI_SUCCESS ==> PrivateEventState(new_s) == PrivateEventState(old_s))
  && ((SdeiIsSupported(old_s) &&
       !AnySharedEventHandlerRunning(old_s) &&
       !AnyInterruptEventBindingRegistered(old_s))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> AnySharedEventRegistered(new_s))
  && (result != SDEI_SUCCESS
    ==> AnyInterruptBoundToEvent(new_s))
  && (result != SDEI_SUCCESS
    ==> !(SharedEventAuxInfoCleared(new_s) && InterruptBindingAuxInfoCleared(new_s)))
  && (result != SDEI_SUCCESS
    ==> PrivateEventState(new_s) != PrivateEventState(old_s))
}
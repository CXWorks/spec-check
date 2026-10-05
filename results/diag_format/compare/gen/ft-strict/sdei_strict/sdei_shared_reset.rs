pub open spec fn sdei_shared_reset_spec(result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (exists|e: SdeiEvent| IsSharedEvent(e) && e.handler_running == true ==> ResultEqual(result, DENIED))
  && (exists|e: SdeiEvent| IsInterruptBoundEvent(e) && IsEventRegistered(e) ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS ==> forall|e: SdeiEvent| IsSharedEvent(e) ==> !IsEventRegistered(e))
  && (result == SDEI_SUCCESS ==> forall|i: Interrupt| !IsInterruptBoundToEvent(i))
  && (result == SDEI_SUCCESS ==> SharedAuxiliaryInfoCleared())
  && (result == SDEI_SUCCESS ==> forall|e: SdeiEvent| IsPrivateEvent(e) ==> PrivateEventStateUnchanged(e))
  && ((IsSdeiSupported(old_s) &&
       !(exists|e: SdeiEvent| IsSharedEvent(e) && e.handler_running == true) &&
       !(exists|e: SdeiEvent| IsInterruptBoundEvent(e) && IsEventRegistered(e)))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> forall|e: SdeiEvent| IsSharedEvent(e) ==> IsEventRegistered(e))
  && (result != SDEI_SUCCESS
    ==> forall|i: Interrupt| IsInterruptBoundToEvent(i))
  && (result != SDEI_SUCCESS
    ==> !SharedAuxiliaryInfoCleared())
  && (result != SDEI_SUCCESS
    ==> forall|e: SdeiEvent| IsPrivateEvent(e) ==> !(PrivateEventStateUnchanged(e)))
}
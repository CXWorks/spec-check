pub open spec fn sdei_private_reset_spec(result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (exists|e: SdeiEvent| IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && HandlerRunning(e) ==> ResultEqual(result, DENIED))
  && (result == SdeiCommandReturnCode::SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SdeiCommandReturnCode::SUCCESS ==> forall|e: SdeiEvent| (IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && !HandlerRunning(e)) ==> EventIsUnregistered(e))
  && (result == SdeiCommandReturnCode::SUCCESS ==> forall|e: SdeiEvent| (IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && HandlerRunning(e)) ==> HandlerUnregisterPending(e))
  && (result == SdeiCommandReturnCode::SUCCESS ==> PrivateEventAuxInfoReset(CallingPe()))
  && (result == SdeiCommandReturnCode::SUCCESS ==> forall|e: SdeiEvent| IsSharedEvent(e) ==> EventStateUnchanged(e))
  && ((SdeiIsSupported(old_s) &&
       !(exists|e: SdeiEvent| IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && HandlerRunning(e)))
    ==> result == SdeiCommandReturnCode::SUCCESS)
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> forall|e: SdeiEvent| (IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && !HandlerRunning(e)) ==> EventIsRegistered(e))
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> forall|e: SdeiEvent| (IsPrivateEvent(e) && EventOwnerPe(e) == CallingPe() && HandlerRunning(e)) ==> !HandlerUnregisterPending(e))
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> !PrivateEventAuxInfoReset(CallingPe()))
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> forall|e: SdeiEvent| IsSharedEvent(e) ==> EventStateUnchanged(e))
}
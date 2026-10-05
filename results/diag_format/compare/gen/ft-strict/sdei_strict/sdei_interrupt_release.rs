pub open spec fn sdei_interrupt_release_spec(event: Int32, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsBoundInterruptEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsPrivateEvent(old_s, event) && !(forall|pe: Pe| IsRegisteredPe(old_s, pe) ==> EventHandlerState(old_s, event, pe) == HANDLER_UNREGISTERED) ==> ResultEqual(result, DENIED))
  && (IsSharedEvent(old_s, event) && SharedEventHandlerState(old_s, event) != HANDLER_UNREGISTERED ==> ResultEqual(result, DENIED))
  && (result == SdeiCommandReturnCode::SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SdeiCommandReturnCode::SUCCESS ==> !IsBoundInterruptEvent(new_s, event))
  && (result == SdeiCommandReturnCode::SUCCESS ==> !IsValidEventNumber(new_s, event))
  && (result == SdeiCommandReturnCode::SUCCESS ==> BindSlotReturnedToPool(new_s, event))
  && (result == SdeiCommandReturnCode::SUCCESS ==> (forall|pe: Pe| IsRegisteredPe(new_s, pe) ==> !IsUnregisterPending(new_s, event, pe)))
  && (result == SdeiCommandReturnCode::SUCCESS ==> InterruptConfigRestoredFromBind(new_s, ReleasedInterrupt(new_s, event)))
  && (result == SdeiCommandReturnCode::SUCCESS && IsPhysicalSdeiInstance(old_s) && SystemUsesGic(old_s) ==> InterruptGroup(new_s, ReleasedInterrupt(new_s, event)) == GROUP1_NON_SECURE)
  && (result == SdeiCommandReturnCode::SUCCESS ==> !IsInterruptEnabledAtController(new_s, ReleasedInterrupt(new_s, event)))
  && ((IsSdeiSupported(old_s) &&
       IsValidEventNumber(old_s, event) &&
       IsBoundInterruptEvent(old_s, event) &&
       !(IsPrivateEvent(old_s, event) && !(forall|pe: Pe| IsRegisteredPe(old_s, pe) ==> EventHandlerState(old_s, event, pe) == HANDLER_UNREGISTERED)) &&
       !(IsSharedEvent(old_s, event) && SharedEventHandlerState(old_s, event) != HANDLER_UNREGISTERED))
    ==> result == SdeiCommandReturnCode::SUCCESS)
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> IsBoundInterruptEvent(new_s, event))
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> IsValidEventNumber(new_s, event))
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> !BindSlotReturnedToPool(new_s, event))
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> (forall|pe: Pe| IsRegisteredPe(new_s, pe) ==> IsUnregisterPending(new_s, event, pe)))
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> InterruptConfigRestoredFromBind(new_s, ReleasedInterrupt(new_s, event)))
  && (result != SdeiCommandReturnCode::SUCCESS
    && (IsPhysicalSdeiInstance(old_s) && SystemUsesGic(old_s))
    ==> InterruptGroup(new_s, ReleasedInterrupt(new_s, event)) != GROUP1_NON_SECURE)
  && (result != SdeiCommandReturnCode::SUCCESS
    ==> IsInterruptEnabledAtController(new_s, ReleasedInterrupt(new_s, event)))
  && (forall (e: Event), (result == SdeiCommandReturnCode::SUCCESS && (IsPrivateEvent(old_s, e) && !(forall|pe: Pe| IsRegisteredPe(old_s, pe) ==> EventHandlerState(old_s, e, pe) == HANDLER_UNREGISTERED))) ==> EventHandlerState(new_s, e, e) == HANDLER_UNREGISTERED)
}
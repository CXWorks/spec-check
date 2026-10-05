pub open spec fn sdei_interrupt_release_spec(event: Int32, result: Result<(), SdeiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsEventBound(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsPrivateEvent(old_s, event) && !(forall pe in RegisteredPes(old_s, event) : HandlerState(old_s, event, pe) == HANDLER_UNREGISTERED) ==> ResultEqual(result, DENIED))
  && (IsSharedEvent(old_s, event) && HandlerState(old_s, event) != HANDLER_UNREGISTERED ==> ResultEqual(result, DENIED))
  && (result == SDEI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SDEI_SUCCESS ==> !IsValidEvent(new_s, event))
  && (result == SDEI_SUCCESS ==> BindSlotOf(new_s, event) is returned to the pool of bind slots)
  && (result == SDEI_SUCCESS ==> InterruptConfig(new_s, BoundInterrupt(new_s, event)) == PreBindInterruptConfig(new_s, BoundInterrupt(new_s, event)))
  && (result == SDEI_SUCCESS ==> !IsInterruptEnabled(new_s, BoundInterrupt(new_s, event)))
  && ((IsSdeiSupported(old_s) &&
       IsValidEvent(old_s, event) &&
       IsEventBound(old_s, event) &&
       !(IsPrivateEvent(old_s, event) && !(forall pe in RegisteredPes(old_s, event) : HandlerState(old_s, event, pe) == HANDLER_UNREGISTERED)) &&
       !(IsSharedEvent(old_s, event) && HandlerState(old_s, event) != HANDLER_UNREGISTERED))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> IsValidEvent(new_s, event))
  && (result != SDEI_SUCCESS
    ==> BindSlotOf(new_s, event) is returned to the pool of bind slots)
  && (result != SDEI_SUCCESS
    ==> InterruptConfig(new_s, BoundInterrupt(new_s, event)) == PreBindInterruptConfig(new_s, BoundInterrupt(new_s, event)))
  && (result != SDEI_SUCCESS
    ==> IsInterruptEnabled(new_s, BoundInterrupt(new_s, event)))
  && (result != SDEI_SUCCESS
    ==> InterruptConfig(new_s, BoundInterrupt(new_s, event)) == InterruptConfig(old_s, BoundInterrupt(old_s, event)))
}
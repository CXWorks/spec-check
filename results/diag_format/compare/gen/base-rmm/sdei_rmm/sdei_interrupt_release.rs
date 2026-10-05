pub open spec fn sdei_interrupt_release_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEvent(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsEventBound(event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsPrivateEvent(event) && !(forall pe in RegisteredPes(event) : HandlerState(event, pe) == HANDLER_UNREGISTERED) ==> ResultEqual(result, DENIED))
    && (IsSharedEvent(event) && HandlerState(event) != HANDLER_UNREGISTERED ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> !IsValidEvent(event))
    && (ResultEqual(result, SUCCESS) ==> BindSlotOf(event) is returned to the pool of bind slots)
    && (ResultEqual(result, SUCCESS) ==> InterruptConfig(BoundInterrupt(event)) == PreBindInterruptConfig(BoundInterrupt(event)))
    && (ResultEqual(result, SUCCESS) ==> !IsInterruptEnabled(BoundInterrupt(event)))
}
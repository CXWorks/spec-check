pub open spec fn sdei_interrupt_release_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidEventNumber(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsBoundInterruptEvent(old_s, event) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsPrivateEvent(event) && !(forall|pe: Pe| IsRegisteredPe(pe) ==> EventHandlerState(event, pe) == HANDLER_UNREGISTERED) ==> ResultEqual(result, DENIED))
    && (IsSharedEvent(event) && SharedEventHandlerState(event) != HANDLER_UNREGISTERED ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> !IsBoundInterruptEvent(new_s, event))
    && (ResultEqual(result, SUCCESS) ==> !IsValidEventNumber(new_s, event))
    && (ResultEqual(result, SUCCESS) ==> BindSlotReturnedToPool(event))
    && (ResultEqual(result, SUCCESS) ==> forall|pe: Pe| IsRegisteredPe(pe) ==> !IsUnregisterPending(new_s, event, pe))
    && (ResultEqual(result, SUCCESS) ==> InterruptConfigRestoredFromBind(new_s, ReleasedInterrupt(event)))
    && (IsPhysicalSdeiInstance() && SystemUsesGic() && ResultEqual(result, SUCCESS) ==> InterruptGroup(new_s, ReleasedInterrupt(event)) == GROUP1_NON_SECURE)
    && (ResultEqual(result, SUCCESS) ==> !IsInterruptEnabledAtController(new_s, ReleasedInterrupt(event)))
}
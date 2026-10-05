pub open spec fn sdei_interrupt_bind_spec(interrupt: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidInterrupt(old_s, interrupt) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsPpi(old_s, interrupt) && !IsSpi(old_s, interrupt) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsOwnedByClient(old_s, interrupt) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (InterruptState(old_s, interrupt) != INACTIVE ==> ResultEqual(result, DENIED))
  && (!IsBound(old_s, interrupt) && NumFreeBindSlots(old_s) == 0 ==> ResultEqual(result, OUT_OF_RESOURCE))
  && (result >= 0 ==> Bits(result, 63, 32) == 0)
  && (result >= 0 ==> IsVendorEventNumber(Bits(result, 31, 0)))
  && (result >= 0 ==> EventIsBoundToInterrupt(new_s, Bits(result, 31, 0), interrupt))
  && (result >= 0 ==> EventPriority(new_s, Bits(result, 31, 0)) == NORMAL_PRIORITY)
  && (result >= 0 && IsPpi(old_s, interrupt) ==> IsPrivateEvent(new_s, Bits(result, 31, 0)))
  && (result >= 0 && IsSpi(old_s, interrupt) ==> IsSharedEvent(new_s, Bits(result, 31, 0)))
  && (result >= 0 ==> forall|pe: PE| EventNumberIsValidOnPe(new_s, Bits(result, 31, 0), pe))
  && (result >= 0 && IsBound(old_s, interrupt) ==> Bits(result, 31, 0) == BoundEventNumber(old_s, interrupt))
  && (result >= 0 ==> InterruptPriorityIsElevated(new_s, interrupt))
  && (result >= 0 ==> InterruptIsManagedByDispatcher(new_s, interrupt))
  && ((SdeiIsSupported(old_s) &&
       IsValidInterrupt(old_s, interrupt) &&
       (IsPpi(old_s, interrupt) || IsSpi(old_s, interrupt)) &&
       IsOwnedByClient(old_s, interrupt) &&
       !(InterruptState(old_s, interrupt) != INACTIVE) &&
       (IsBound(old_s, interrupt) || NumFreeBindSlots(old_s) != 0))
    ==> result >= 0)
  && (result < 0
    ==> EventIsBoundToInterrupt(new_s, Bits(result, 31, 0), interrupt))
  && (result < 0
    ==> EventPriority(new_s, Bits(result, 31, 0)) == NORMAL_PRIORITY)
  && (result < 0
    ==> InterruptPriorityIsElevated(new_s, interrupt))
  && (result < 0
    ==> InterruptIsManagedByDispatcher(new_s, interrupt))
}
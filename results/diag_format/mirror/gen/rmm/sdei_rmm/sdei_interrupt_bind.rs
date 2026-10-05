pub open spec fn sdei_interrupt_bind_spec(interrupt: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
  (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidInterrupt(old_s, interrupt) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsInterruptAllowedForBinding(old_s, interrupt) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (InterruptState(old_s, interrupt) != INACTIVE ==> ResultEqual(result, DENIED))
  && (!IsInterruptBound(old_s, interrupt) && !BindSlotAvailable(old_s) ==> ResultEqual(result, OUT_OF_RESOURCE))
  && (result >= 0 ==> result[63:32] == 0)
  && (result >= 0 ==> result[31:0] == BoundEventNumber(new_s, interrupt))
  && (result >= 0 ==> IsInterruptBound(new_s, interrupt))
  && (result >= 0 ==> EventPriority(new_s, result[31:0]) == NORMAL)
  && (result >= 0 && IsPpi(old_s, interrupt) ==> IsPrivateEvent(new_s, result[31:0]))
  && (result >= 0 && IsSpi(old_s, interrupt) ==> IsSharedEvent(new_s, result[31:0]))
  && (result >= 0 && IsInterruptBound(old_s, interrupt) ==> result[31:0] == PreviousBoundEventNumber(new_s, interrupt))
  && (result >= 0 ==> IsVendorEventNumber(new_s, result[31:0]))
  && (result >= 0 ==> IsInterruptPriorityElevated(new_s, interrupt))
  && ((SdeiIsSupported(old_s) &&
       IsValidInterrupt(old_s, interrupt) &&
       IsInterruptAllowedForBinding(old_s, interrupt) &&
       !(InterruptState(old_s, interrupt) != INACTIVE) &&
       (IsInterruptBound(old_s, interrupt) || BindSlotAvailable(old_s)))
    ==> result >= 0)
  && (result < 0
    ==> IsInterruptBound(new_s, interrupt))
  && (result < 0
    ==> InterruptState(new_s, interrupt) == InterruptState(old_s, interrupt))
  && (result < 0
    ==> InterruptPriority(new_s, interrupt) == InterruptPriority(old_s, interrupt))
}
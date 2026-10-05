pub open spec fn sdei_interrupt_bind_spec(interrupt: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidInterrupt(old_s, interrupt) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsInterruptAllowedForBinding(old_s, interrupt) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (InterruptState(old_s, interrupt) != INACTIVE ==> ResultEqual(result, DENIED))
    && ((!IsInterruptBound(old_s, interrupt) && !BindSlotAvailable(old_s)) ==> ResultEqual(result, OUT_OF_RESOURCE))
    && ((SdeiIsSupported(old_s)
        && IsValidInterrupt(old_s, interrupt)
        && IsInterruptAllowedForBinding(old_s, interrupt)
        && InterruptState(old_s, interrupt) == INACTIVE
        && (IsInterruptBound(old_s, interrupt) || BindSlotAvailable(old_s)))
        ==> (
            ((result as u64) >> 32u64) == 0u64
            && ((result as u64) & 0xFFFF_FFFFu64) == BoundEventNumber(new_s, interrupt)
            && IsInterruptBound(new_s, interrupt)
            && EventPriority(new_s, (result as u64) & 0xFFFF_FFFFu64) == NORMAL
            && (IsPpi(old_s, interrupt) ==> IsPrivateEvent(new_s, (result as u64) & 0xFFFF_FFFFu64))
            && (IsSpi(old_s, interrupt) ==> IsSharedEvent(new_s, (result as u64) & 0xFFFF_FFFFu64))
            && (IsInterruptBound(old_s, interrupt) ==> ((result as u64) & 0xFFFF_FFFFu64) == PreviousBoundEventNumber(old_s, interrupt))
            && IsVendorEventNumber(new_s, (result as u64) & 0xFFFF_FFFFu64)
            && IsInterruptPriorityElevated(new_s, interrupt)
        ))
}

pub open spec fn clock_rate_set__3_6_2_7_spec(flags: UInt32, clock_id: UInt32, rate_low: UInt32, rate_high: UInt32, status: ScmiStatusCode, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id as int) ==> (status == NOT_FOUND && new_s == old_s))
    && ((ClockExists(old_s, clock_id as int)
        && ((flags & 0xFFFF_FFF0u32) != 0u32
            || !ClockRateSupported(old_s, clock_id as int, (rate_high as int) * 0x1_0000_0000 + (rate_low as int))))
        ==> (status == INVALID_PARAMETERS && new_s == old_s))
    && ((ClockExists(old_s, clock_id as int)
        && (flags & 0xFFFF_FFF0u32) == 0u32
        && ClockRateSupported(old_s, clock_id as int, (rate_high as int) * 0x1_0000_0000 + (rate_low as int))
        && (flags & 0x1u32) == 0x1u32
        && ClockAsyncRateChangesPendingFull(old_s))
        ==> (status == BUSY && new_s == old_s))
    && ((ClockExists(old_s, clock_id as int)
        && (flags & 0xFFFF_FFF0u32) == 0u32
        && ClockRateSupported(old_s, clock_id as int, (rate_high as int) * 0x1_0000_0000 + (rate_low as int))
        && !((flags & 0x1u32) == 0x1u32 && ClockAsyncRateChangesPendingFull(old_s))
        && ClockRateSetDeniedByDependencies(old_s, clock_id as int))
        ==> (status == DENIED && new_s == old_s))
    && ((ClockExists(old_s, clock_id as int)
        && (flags & 0xFFFF_FFF0u32) == 0u32
        && ClockRateSupported(old_s, clock_id as int, (rate_high as int) * 0x1_0000_0000 + (rate_low as int))
        && !((flags & 0x1u32) == 0x1u32 && ClockAsyncRateChangesPendingFull(old_s))
        && !ClockRateSetDeniedByDependencies(old_s, clock_id as int))
        ==> (status == SUCCESS
            && ((flags & 0x1u32) == 0u32 ==>
                ClockRate(new_s, clock_id as int) == ClockRoundedRate(old_s, clock_id as int, (rate_high as int) * 0x1_0000_0000 + (rate_low as int), (flags & 0x8u32) == 0x8u32, (flags & 0x4u32) == 0x4u32))
            && ((flags & 0x1u32) == 0x1u32 ==>
                (ClockRateChangeQueued(new_s, clock_id as int, ClockRoundedRate(old_s, clock_id as int, (rate_high as int) * 0x1_0000_0000 + (rate_low as int), (flags & 0x8u32) == 0x8u32, (flags & 0x4u32) == 0x4u32))
                && (ClockRateSetDelayedResponsePending(new_s, clock_id as int) == ((flags & 0x2u32) == 0u32))))))
    && (status == SUCCESS ==>
        (ClockExists(old_s, clock_id as int)
        && (flags & 0xFFFF_FFF0u32) == 0u32
        && ClockRateSupported(old_s, clock_id as int, (rate_high as int) * 0x1_0000_0000 + (rate_low as int))))
}

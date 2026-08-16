pub open spec fn rmi_rtt_set_ripas_spec(
    result: Result<(), RmiStatusCode>,
    out_top: Address,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(
        old_s,
        rec_ptr,
    ).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Rec(old_s, rec_ptr).state
        == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC)) && (Rec(old_s, rec_ptr).owner != rd
        ==> ResultEqual(result, RMI_ERROR_REC)) && ((top as int) <= (base as int) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (base != Rec(old_s, rec_ptr).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    top as int) > (Rec(old_s, rec_ptr).ripas_top as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, rd, base).level as int) && RttWalk(
        old_s,
        rd,
        base,
    ).rtte.ripas != Rec(old_s, rec_ptr).ripas_value) ==> ResultEqual(
        result,
        RMI_ERROR_RTT(RttWalk(old_s, rd, base).level as int),
    )) && (((base as int) == (RttSkipEntriesWithRipas(
        old_s,
        Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr),
        RttWalk(old_s, rd, base).level as int,
        base,
        top,
        Rec(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED,
    ) as int) && RttWalk(old_s, rd, base).rtte.ripas != Rec(old_s, rec_ptr).ripas_value)
        ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, rd, base).level as int))) && ((
    AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, rec_ptr) && PaIsDelegable(old_s, rec_ptr) && Granule(
        old_s,
        rec_ptr,
    ).state == REC && Rec(old_s, rec_ptr).state != REC_RUNNING && Rec(old_s, rec_ptr).owner == rd
        && (top as int) > (base as int) && base == Rec(old_s, rec_ptr).ripas_addr && (top as int)
        <= (Rec(old_s, rec_ptr).ripas_top as int) && AddrIsGranuleAligned(old_s, top) && (
    AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, rd, base).level as int) || RttWalk(
        old_s,
        rd,
        base,
    ).rtte.ripas == Rec(old_s, rec_ptr).ripas_value) && ((base as int) != (RttSkipEntriesWithRipas(
        old_s,
        Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr),
        RttWalk(old_s, rd, base).level as int,
        base,
        top,
        Rec(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED,
    ) as int) || RttWalk(old_s, rd, base).rtte.ripas == Rec(old_s, rec_ptr).ripas_value)) ==> (
    result.is_Ok() && RttEntriesInRangeRipas(
        new_s,
        Rtt(new_s, RttWalk(old_s, rd, base).rtt_addr),
        RttWalk(old_s, rd, base).level as int,
        base,
        RttSkipEntriesWithRipas(
            old_s,
            Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr),
            RttWalk(old_s, rd, base).level as int,
            base,
            top,
            Rec(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED,
        ),
        Rec(old_s, rec_ptr).ripas_value,
    ) && Rec(new_s, rec_ptr).ripas_addr == MinAddress(
        old_s,
        top,
        RttSkipEntriesWithRipas(
            old_s,
            Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr),
            RttWalk(old_s, rd, base).level as int,
            base,
            top,
            Rec(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED,
        ),
    ) && out_top == MinAddress(
        old_s,
        top,
        RttSkipEntriesWithRipas(
            old_s,
            Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr),
            RttWalk(old_s, rd, base).level as int,
            base,
            top,
            Rec(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED,
        ),
    )))
}
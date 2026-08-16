pub open spec fn rmi_rtt_set_ripas_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
    out_top: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let walk = RttWalk(old_s, rd, base);
    let rec = Rec(old_s, rec_ptr);
    let ripas = walk.rtte.ripas;
    let walk_top = RttSkipEntriesWithRipas(
        old_s,
        Rtt(old_s, walk.rtt_addr),
        walk.level as int,
        base,
        top,
        rec.ripas_destroyed != CHANGE_DESTROYED,
    );
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
    ).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rec.state == REC_RUNNING
        ==> ResultEqual(result, RMI_ERROR_REC)) && (rec.owner != rd ==> ResultEqual(
        result,
        RMI_ERROR_REC,
    )) && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT)) && (base != rec.ripas_addr
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (top > rec.ripas_top ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((!AddrIsRttLevelAligned(old_s, base, walk.level as int) && ripas != rec.ripas_value)
        ==> ResultEqual(result, RMI_ERROR_RTT)) && (!AddrIsGranuleAligned(old_s, top)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((base == walk_top && ripas != rec.ripas_value)
        ==> ResultEqual(result, RMI_ERROR_RTT)) && (AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state == RD && AddrIsGranuleAligned(
        old_s,
        rec_ptr,
    ) && PaIsDelegable(old_s, rec_ptr) && Granule(old_s, rec_ptr).state == REC && rec.state
        != REC_RUNNING && rec.owner == rd && top > base && base == rec.ripas_addr && top
        <= rec.ripas_top && (AddrIsRttLevelAligned(old_s, base, walk.level as int) || ripas
        == rec.ripas_value) && AddrIsGranuleAligned(old_s, top) && (base != walk_top || ripas
        == rec.ripas_value) ==> (result.is_Ok() && RttEntriesInRangeRipas(
        new_s,
        Rtt(new_s, walk.rtt_addr),
        walk.level as int,
        base,
        walk_top,
        rec.ripas_value,
    ) && Rec(new_s, rec_ptr).ripas_addr == MinAddress(old_s, top, walk_top) && out_top
        == MinAddress(old_s, top, walk_top)))
}
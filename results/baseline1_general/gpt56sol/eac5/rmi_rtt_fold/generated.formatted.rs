pub open spec fn rmi_rtt_fold_spec(
    result: Result<(), RmiStatusCode>,
    rtt: Address,
    rd: Address,
    ipa: Address,
    level: Int64,
    old_s: S,
    new_s: S,
) -> bool {
    let walk = RttWalk(old_s, rd, ipa);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold = RttFold(old_s, Rtt(old_s, walk.rtte.addr));
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(
        old_s,
        rd,
        level as int,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(
        old_s,
        ipa,
        (level - 1) as int,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((ipa as int) >= (1int << (Realm(
        old_s,
        rd,
    ).ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (walk.level < (level
        - 1) as int ==> ResultEqual(result, RMI_ERROR_RTT_AUX(walk.level as int))) && (
    walk.rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT_AUX(walk.level as int))) && (
    !RttIsHomogeneous(old_s, Rtt(old_s, walk.rtte.addr)) ==> ResultEqual(
        result,
        RMI_ERROR_RTT_AUX(level as int),
    )) && (AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state
        == RD && RttLevelIsValid(old_s, rd, level as int) && !RttLevelIsStarting(
        old_s,
        rd,
        level as int,
    ) && AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int) && (ipa as int) < (1int << (Realm(
        old_s,
        rd,
    ).ipa_width as int)) && walk.level >= (level - 1) as int && walk.rtte.state == TABLE
        && RttIsHomogeneous(old_s, Rtt(old_s, walk.rtte.addr)) ==> (result.is_Ok() && RttEntry(
        new_s,
        walk.rtt_addr,
        entry_idx,
    ).state == fold.state && ((fold.state != UNASSIGNED && fold.state != UNASSIGNED_NS)
        ==> RttEntry(new_s, walk.rtt_addr, entry_idx).addr == fold.addr) && ((fold.state == ASSIGNED
        || fold.state == ASSIGNED_NS) ==> (RttEntry(new_s, walk.rtt_addr, entry_idx).MemAttr
        == fold.MemAttr && RttEntry(new_s, walk.rtt_addr, entry_idx).S2AP == fold.S2AP && RttEntry(
        new_s,
        walk.rtt_addr,
        entry_idx,
    ).SH == fold.SH)) && (AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> RttEntry(
        new_s,
        walk.rtt_addr,
        entry_idx,
    ).ripas == fold.ripas) && Granule(new_s, walk.rtte.addr).state == DELEGATED && rtt
        == walk.rtte.addr))
}
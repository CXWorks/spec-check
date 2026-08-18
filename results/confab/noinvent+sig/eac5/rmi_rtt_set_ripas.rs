pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let walk = RttWalk(old_s, rd, base);
    let realm = Realm(old_s, rd);
    let walk_top = RttSkipEntriesWithRipas(
        old_s,
        Rtt(old_s, walk.rtt_addr),
        walk.level,
        base,
        top,
        Rec(old_s, rec).ripas_destroyed != CHANGE_DESTROYED,
    );
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Rec(old_s, rec).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (Rec(old_s, rec).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != Rec(old_s, rec).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (top > Rec(old_s, rec).ripas_top ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, base, walk.level) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((top < RttUpperBound(old_s, base, walk.level, realm.ipa_width as int)
            && RttEntryHasRipas(old_s, RttEntry(old_s, walk.rtt_addr, RttEntryIndex(old_s, top, walk.level)))
            && !AddrIsRttLevelAligned(old_s, top, walk.level))
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && Granule(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, rec)
            && PaIsDelegable(old_s, rec)
            && Granule(old_s, rec).state == REC
            && Rec(old_s, rec).state != REC_RUNNING
            && Rec(old_s, rec).owner == rd
            && top > base
            && base == Rec(old_s, rec).ripas_addr
            && top <= Rec(old_s, rec).ripas_top
            && AddrIsGranuleAligned(old_s, top)
            && AddrIsRttLevelAligned(old_s, base, walk.level)
            && !(top < RttUpperBound(old_s, base, walk.level, realm.ipa_width as int)
                && RttEntryHasRipas(old_s, RttEntry(old_s, walk.rtt_addr, RttEntryIndex(old_s, top, walk.level)))
                && !AddrIsRttLevelAligned(old_s, top, walk.level)))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(
                new_s,
                Rtt(new_s, walk.rtt_addr),
                walk.level,
                base,
                walk_top,
                Rec(old_s, rec).ripas_value,
            )
            && Rec(new_s, rec).ripas_addr == walk_top
            && out_top == walk_top))
}
pub open spec fn rmi_rtt_set_ripas_spec(
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
    rd: Address,
    rec: Address,
    base: Address,
    top: Address,
    out_top: Address,
) -> bool {
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
    && (!AddrIsRttLevelAligned(
            old_s,
            base,
            RttWalk(old_s, rd, base).level as int,
        ) ==> ResultEqual(
            result,
            RMI_ERROR_RTT_AUX(RttWalk(old_s, rd, base).level as int),
        ))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((top < RttUpperBound(
                old_s,
                base,
                RttWalk(old_s, rd, base).level as int,
                Realm(old_s, rd).ipa_width as int,
            ))
            && RttEntryHasRipas(
                old_s,
                RttEntry(
                    old_s,
                    RttWalk(old_s, rd, base).rtt_addr,
                    RttEntryIndex(
                        old_s,
                        top,
                        RttWalk(old_s, rd, base).level as int,
                    ),
                ),
            )
            && !AddrIsRttLevelAligned(
                old_s,
                top,
                RttWalk(old_s, rd, base).level as int,
            ))
        ==> ResultEqual(
            result,
            RMI_ERROR_RTT_AUX(RttWalk(old_s, rd, base).level as int),
        ))
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
            && AddrIsRttLevelAligned(
                old_s,
                base,
                RttWalk(old_s, rd, base).level as int,
            )
            && AddrIsGranuleAligned(old_s, top)
            && !((top < RttUpperBound(
                        old_s,
                        base,
                        RttWalk(old_s, rd, base).level as int,
                        Realm(old_s, rd).ipa_width as int,
                    ))
                    && RttEntryHasRipas(
                        old_s,
                        RttEntry(
                            old_s,
                            RttWalk(old_s, rd, base).rtt_addr,
                            RttEntryIndex(
                                old_s,
                                top,
                                RttWalk(old_s, rd, base).level as int,
                            ),
                        ),
                    )
                    && !AddrIsRttLevelAligned(
                        old_s,
                        top,
                        RttWalk(old_s, rd, base).level as int,
                    )))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(
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
                    Rec(old_s, rec).ripas_destroyed != CHANGE_DESTROYED,
                ),
                Rec(old_s, rec).ripas_value,
            )
            && Rec(new_s, rec).ripas_addr
                == RttSkipEntriesWithRipas(
                    old_s,
                    Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr),
                    RttWalk(old_s, rd, base).level as int,
                    base,
                    top,
                    Rec(old_s, rec).ripas_destroyed != CHANGE_DESTROYED,
                )
            && out_top
                == RttSkipEntriesWithRipas(
                    old_s,
                    Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr),
                    RttWalk(old_s, rd, base).level as int,
                    base,
                    top,
                    Rec(old_s, rec).ripas_destroyed != CHANGE_DESTROYED,
                )))
}
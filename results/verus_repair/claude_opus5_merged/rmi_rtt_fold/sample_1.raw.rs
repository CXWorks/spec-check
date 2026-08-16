pub open spec fn rmi_rtt_fold_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, level: int, rtt: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, rd, level) || RttLevelIsStarting(old_s, rd, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1int << Realm(old_s, rd).ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, rd, ipa).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, rd, ipa).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT))
    && (!RttIsHomogeneous(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && Granule(old_s, rd).state == RD
            && RttLevelIsValid(old_s, rd, level)
            && !RttLevelIsStarting(old_s, rd, level)
            && AddrIsRttLevelAligned(old_s, ipa, level - 1)
            && (ipa as int) < (1int << Realm(old_s, rd).ipa_width)
            && RttWalk(old_s, rd, ipa).level >= level - 1
            && RttWalk(old_s, rd, ipa).rtte.state == TABLE
            && RttIsHomogeneous(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)))
        ==> (result.is_Ok()
            && RttWalk(new_s, rd, ipa).rtte.state == RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).state
            && ((RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).state != UNASSIGNED
                    && RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).state != UNASSIGNED_NS)
                ==> RttWalk(new_s, rd, ipa).rtte.addr == RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).addr)
            && ((RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).state == ASSIGNED
                    || RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).state == ASSIGNED_NS)
                ==> (RttWalk(new_s, rd, ipa).rtte.MemAttr == RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).MemAttr
                    && RttWalk(new_s, rd, ipa).rtte.S2AP == RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).S2AP
                    && RttWalk(new_s, rd, ipa).rtte.SH == RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).SH))
            && (AddrIsProtected(old_s, ipa, Realm(old_s, rd))
                ==> RttWalk(new_s, rd, ipa).rtte.ripas == RttFold(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)).ripas)
            && Granule(new_s, RttWalk(old_s, rd, ipa).rtte.addr).state == DELEGATED
            && rtt == RttWalk(old_s, rd, ipa).rtte.addr))
}
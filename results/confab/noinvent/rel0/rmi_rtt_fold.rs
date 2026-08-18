pub open spec fn rmi_rtt_fold_spec(result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S, rd: Address, ipa: Address, level: int) -> bool {
    let walk = RttWalk(old_s, rd, ipa);
    let new_walk = RttWalk(new_s, rd, ipa);
    let fold = RttFold(old_s, Rtt(old_s, walk.rtte.addr));
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, rd, level) || RttLevelIsStarting(old_s, rd, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1int << Realm(old_s, rd).ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT))
    && (!RttIsHomogeneous(old_s, Rtt(old_s, walk.rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && Granule(old_s, rd).state == RD
            && RttLevelIsValid(old_s, rd, level)
            && !RttLevelIsStarting(old_s, rd, level)
            && AddrIsRttLevelAligned(old_s, ipa, level - 1)
            && (ipa as int) < (1int << Realm(old_s, rd).ipa_width)
            && walk.level >= level - 1
            && walk.rtte.state == TABLE
            && RttIsHomogeneous(old_s, Rtt(old_s, walk.rtte.addr)))
        ==> (result.is_Ok()
            && new_walk.rtte.state == fold.state
            && ((fold.state != UNASSIGNED && fold.state != UNASSIGNED_NS)
                    ==> new_walk.rtte.addr == fold.addr)
            && ((fold.state == ASSIGNED || fold.state == ASSIGNED_NS)
                    ==> (new_walk.rtte.MemAttr == fold.MemAttr
                        && new_walk.rtte.S2AP == fold.S2AP))
            && (AddrIsProtected(old_s, ipa, Realm(old_s, rd))
                    ==> new_walk.rtte.ripas == fold.ripas)
            && Granule(new_s, walk.rtte.addr).state == DELEGATED
            && rtt == walk.rtte.addr))
}
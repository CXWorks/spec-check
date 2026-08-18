pub open spec fn rmi_rtt_read_entry_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, level: i64, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    let walk = RttWalk(old_s, rd, ipa);
    let rtte = RttEntryFromDescriptor(old_s, desc);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1int << (Realm(old_s, rd).ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && Granule(old_s, rd).state == RD
            && RttLevelIsValid(old_s, rd, level as int)
            && AddrIsRttLevelAligned(old_s, ipa, level as int)
            && (ipa as int) < (1int << (Realm(old_s, rd).ipa_width as int)))
        ==> (result.is_Ok()
            && state == RttEntryState(old_s, walk.rtte.state)
            && ((walk.rtte.state == UNASSIGNED || walk.rtte.state == UNASSIGNED_NS)
                ==> (rtte.MemAttr == 0 && rtte.S2AP == 0 && rtte.addr == 0))
            && ((walk.rtte.state == ASSIGNED || walk.rtte.state == TABLE)
                ==> (rtte.MemAttr == 0 && rtte.S2AP == 0 && rtte.addr == walk.rtte.addr))
            && (walk.rtte.state == ASSIGNED_NS
                ==> (rtte.MemAttr == walk.rtte.MemAttr
                    && rtte.S2AP == walk.rtte.S2AP
                    && rtte.addr == walk.rtte.addr))
            && ((walk.rtte.state == UNASSIGNED || walk.rtte.state == ASSIGNED)
                ==> ripas == RipasToRmi(old_s, walk.rtte.ripas))
            && ((walk.rtte.state == UNASSIGNED_NS || walk.rtte.state == ASSIGNED_NS)
                ==> ripas == RMI_EMPTY)
            && new_s == old_s))
}
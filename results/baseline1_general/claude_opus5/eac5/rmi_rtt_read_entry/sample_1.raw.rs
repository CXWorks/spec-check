pub open spec fn rmi_rtt_read_entry_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, level: u64, walk_level: u64, state: RmiRttEntryState, desc: u64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1int << (Realm(old_s, rd).ipa_width as u64)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && Granule(old_s, rd).state == RD
            && RttLevelIsValid(old_s, rd, level as int)
            && AddrIsRttLevelAligned(old_s, ipa, level as int)
            && (ipa as int) < (1int << (Realm(old_s, rd).ipa_width as u64)))
        ==> (result.is_Ok()
            && walk_level as int == RttWalk(old_s, rd, ipa).level
            && state == RttEntryState(old_s, RttWalk(old_s, rd, ipa).rtte.state)
            && ((RttWalk(old_s, rd, ipa).rtte.state == UNASSIGNED
                    || RttWalk(old_s, rd, ipa).rtte.state == UNASSIGNED_NS)
                ==> (RttEntryFromDescriptor(old_s, desc).MemAttr == 0
                    && RttEntryFromDescriptor(old_s, desc).S2AP == 0
                    && RttEntryFromDescriptor(old_s, desc).SH == 0
                    && RttEntryFromDescriptor(old_s, desc).addr == 0))
            && ((RttWalk(old_s, rd, ipa).rtte.state == ASSIGNED
                    || RttWalk(old_s, rd, ipa).rtte.state == TABLE)
                ==> (RttEntryFromDescriptor(old_s, desc).MemAttr == 0
                    && RttEntryFromDescriptor(old_s, desc).S2AP == 0
                    && RttEntryFromDescriptor(old_s, desc).SH == 0
                    && RttEntryFromDescriptor(old_s, desc).addr == RttWalk(old_s, rd, ipa).rtte.addr))
            && (RttWalk(old_s, rd, ipa).rtte.state == ASSIGNED_NS
                ==> (RttEntryFromDescriptor(old_s, desc).MemAttr == RttWalk(old_s, rd, ipa).rtte.MemAttr
                    && RttEntryFromDescriptor(old_s, desc).S2AP == RttWalk(old_s, rd, ipa).rtte.S2AP
                    && RttEntryFromDescriptor(old_s, desc).SH == RttWalk(old_s, rd, ipa).rtte.SH
                    && RttEntryFromDescriptor(old_s, desc).addr == RttWalk(old_s, rd, ipa).rtte.addr))
            && ((RttWalk(old_s, rd, ipa).rtte.state != UNASSIGNED
                    && RttWalk(old_s, rd, ipa).rtte.state != ASSIGNED)
                ==> ripas == RMI_EMPTY)))
    && new_s == old_s
}
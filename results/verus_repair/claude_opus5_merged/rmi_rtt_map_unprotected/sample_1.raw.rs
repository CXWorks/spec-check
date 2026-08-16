pub open spec fn rmi_rtt_map_unprotected_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, level: i64, desc: u64, old_s: S, new_s: S) -> bool {
    let walk = RttWalk(old_s, rd, ipa);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let rtte = RttEntryFromDescriptor(old_s, desc);
    (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, rtte.addr, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as u64) >= (1u64 << Realm(old_s, rd).ipa_width) || AddrIsProtected(old_s, ipa, Realm(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level as int ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((RttDescriptorIsValidForUnprotected(old_s, desc)
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsBlockOrPage(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, rtte.addr, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, level as int)
        && (ipa as u64) < (1u64 << Realm(old_s, rd).ipa_width)
        && !AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && walk.level >= level as int
        && walk.rtte.state == UNASSIGNED_NS)
        ==> (result.is_Ok()
            && RttEntry(new_s, walk.rtt_addr, entry_idx).state == ASSIGNED_NS
            && RttEntry(new_s, walk.rtt_addr, entry_idx).MemAttr == rtte.MemAttr
            && RttEntry(new_s, walk.rtt_addr, entry_idx).S2AP == rtte.S2AP
            && RttEntry(new_s, walk.rtt_addr, entry_idx).SH == rtte.SH
            && RttEntry(new_s, walk.rtt_addr, entry_idx).addr == rtte.addr))
}
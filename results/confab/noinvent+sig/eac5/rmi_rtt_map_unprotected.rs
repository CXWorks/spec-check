pub open spec fn rmi_rtt_map_unprotected_spec(rd: Address, ipa: Address, level: Int64, desc: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa >= (1u64 << (Realm(old_s, rd).ipa_width as u64)) || AddrIsProtected(old_s, ipa, Realm(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, rd, ipa).level < level as int ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, rd, ipa).rtte.state != UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((RttDescriptorIsValidForUnprotected(old_s, desc)
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsBlockOrPage(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, level as int)
        && ipa < (1u64 << (Realm(old_s, rd).ipa_width as u64))
        && !AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && RttWalk(old_s, rd, ipa).level >= level as int
        && RttWalk(old_s, rd, ipa).rtte.state == UNASSIGNED_NS)
        ==> (result.is_Ok()
            && RttWalk(new_s, rd, ipa).rtte.state == ASSIGNED_NS
            && RttWalk(new_s, rd, ipa).rtte.MemAttr == RttEntryFromDescriptor(old_s, desc).MemAttr
            && RttWalk(new_s, rd, ipa).rtte.S2AP == RttEntryFromDescriptor(old_s, desc).S2AP
            && RttWalk(new_s, rd, ipa).rtte.SH == RttEntryFromDescriptor(old_s, desc).SH
            && RttWalk(new_s, rd, ipa).rtte.addr == RttEntryFromDescriptor(old_s, desc).addr))
}
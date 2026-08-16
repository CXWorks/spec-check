pub open spec fn rmi_rtt_map_unprotected_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    ipa: Address,
    level: Int64,
    desc: Bits64,
    old_s: S,
    new_s: S,
) -> bool {
    (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE
        && (RttEntryFromDescriptor(old_s, desc).addr as int) >= 0x1_0000_0000_0000) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (((ipa as int) >= (1int << Realm(old_s, rd).ipa_width) || AddrIsProtected(
        old_s,
        ipa,
        Realm(old_s, rd),
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((RttWalk(old_s, rd, ipa).level < level as int)
        ==> ResultEqual(result, RMI_ERROR_RTT)) && ((RttWalk(old_s, rd, ipa).rtte.state
        != UNASSIGNED_NS) ==> ResultEqual(result, RMI_ERROR_RTT)) && ((
    RttDescriptorIsValidForUnprotected(old_s, desc) && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state == RD && RttLevelIsBlockOrPage(
        old_s,
        rd,
        level as int,
    ) && AddrIsRttLevelAligned(old_s, RttEntryFromDescriptor(old_s, desc).addr, level as int) && !(
    Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE && (RttEntryFromDescriptor(old_s, desc).addr as int)
        >= 0x1_0000_0000_0000) && AddrIsRttLevelAligned(old_s, ipa, level as int) && (ipa as int)
        < (1int << Realm(old_s, rd).ipa_width) && !AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && RttWalk(old_s, rd, ipa).level >= level as int && RttWalk(old_s, rd, ipa).rtte.state
        == UNASSIGNED_NS) ==> (result.is_Ok() && RttEntry(
        new_s,
        RttWalk(old_s, rd, ipa).rtt_addr,
        RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level),
    ).state == ASSIGNED_NS && RttEntry(
        new_s,
        RttWalk(old_s, rd, ipa).rtt_addr,
        RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level),
    ).MemAttr == RttEntryFromDescriptor(old_s, desc).MemAttr && RttEntry(
        new_s,
        RttWalk(old_s, rd, ipa).rtt_addr,
        RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level),
    ).S2AP == RttEntryFromDescriptor(old_s, desc).S2AP && RttEntry(
        new_s,
        RttWalk(old_s, rd, ipa).rtt_addr,
        RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level),
    ).addr == RttEntryFromDescriptor(old_s, desc).addr))
}
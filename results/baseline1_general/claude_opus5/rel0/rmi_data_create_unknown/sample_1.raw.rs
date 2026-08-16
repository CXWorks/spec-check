pub open spec fn rmi_data_create_unknown_spec(result: Result<(), RmiStatusCode>, rd: Address, data: Address, ipa: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data as int) >= 0x1_0000_0000_0000))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, rd, ipa).level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, rd, ipa).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, data)
        && PaIsDelegable(old_s, data)
        && Granule(old_s, data).state == DELEGATED
        && !((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data as int) >= 0x1_0000_0000_0000))
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && RttWalk(old_s, rd, ipa).level >= RMM_RTT_PAGE_LEVEL
        && RttWalk(old_s, rd, ipa).rtte.state == UNASSIGNED)
        ==> (result.is_Ok()
            && Granule(new_s, data).state == DATA
            && RttEntry(new_s, RttWalk(old_s, rd, ipa).rtt_addr, RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level)).state == ASSIGNED
            && RttEntry(new_s, RttWalk(old_s, rd, ipa).rtt_addr, RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level)).addr == data))
}
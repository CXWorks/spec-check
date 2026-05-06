pub open spec fn RMI_DATA_CREATE_UNKNOWN_spec(old_s: S, new_s: S, rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let old_data_granule = GranuleAt(old_s, data);
    let new_data_granule = GranuleAt(new_s, data);
    let old_rtt = RttAt(old_s, walk.rtt_addr);
    let new_rtt = RttAt(new_s, walk.rtt_addr);
    let old_rtte = RttEntryAt(old_s, old_rtt, entry_idx);
    let new_rtte = RttEntryAt(new_s, new_rtt, entry_idx);
    
    ((!AddrIsGranuleAligned(data)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!PaIsDelegableDram(old_s, data)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((old_data_granule.state != DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (((realm.feat_lpa2 == FEATURE_FALSE) && (data as int >= (1 << 48))) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsGranuleAligned(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!PaIsDelegable(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((old_s.granules[rd].state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsGranuleAligned(ipa)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((walk.level < RMM_RTT_PAGE_LEVEL as int) ==> ResultEqual(result, RMI_ERROR_RTT)) &&
    ((old_rtte.state != UNASSIGNED) ==> ResultEqual(result, RMI_ERROR_RTT)) &&
    (result.is_Ok() ==> (
        new_data_granule.state == DATA &&
        new_rtte.state == ASSIGNED &&
        new_rtte.addr == data &&
        new_rtte.attr_prot == MEMATTR_CACHEABLE &&
        new_rtte.sh == SHAREABILITY_INNER
    ))
}
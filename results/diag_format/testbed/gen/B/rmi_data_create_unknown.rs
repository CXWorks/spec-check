pub open spec fn rmi_data_create_unknown_spec(rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level as int);
    let rtte = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx);
    let rtte_new = RttEntryAt(RttAt(new_s, walk.rtt_addr), entry_idx);
    let granule_data_old = GranuleAt(old_s, data);
    let granule_data_new = GranuleAt(new_s, data);
    let granule_rd_old = GranuleAt(old_s, rd);
    let granule_rd_new = GranuleAt(new_s, rd);
    let data_aligned = AddrIsGranuleAligned(data);
    let rd_aligned = AddrIsGranuleAligned(rd);
    let ipa_aligned = AddrIsGranuleAligned(ipa);
    let data_delegable = PaIsDelegableDram(data);
    let rd_delegable = PaIsDelegable(rd);
    let rd_state_ok = GranuleAt(old_s, rd).state == RD;
    let data_state_ok = GranuleAt(old_s, data).state == DELEGATED;
    let lpa2_ok = realm.feat_lpa2 == FEATURE_FALSE || data < pow2(48);
    let rtt_level_ok = walk.level >= RMM_RTT_PAGE_LEVEL;
    let rtte_state_ok = rtte.state == UNASSIGNED;
    let ipa_protected = AddrIsProtected(old_s, ipa, realm);
    (!data_aligned ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!data_delegable ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!data_state_ok ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!lpa2_ok ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!rd_aligned ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!rd_delegable ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!rd_state_ok ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!ipa_aligned ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!ipa_protected ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!rtt_level_ok ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (!rtte_state_ok ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (result.is_Ok() ==> (
        granule_data_new.state == DATA
        && GranuleContentsWiped(data)
        && rtte_new.state == ASSIGNED
        && rtte_new.addr == data
        && rtte_new.attr_prot == MEMATTR_CACHEABLE
        && rtte_new.sh == SHAREABILITY_INNER
    ))
}
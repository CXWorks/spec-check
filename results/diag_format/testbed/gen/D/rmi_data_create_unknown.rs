pub open spec fn rmi_data_create_unknown_spec(rd: Address, data: Address, ipa: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let entry_idx = RttEntryIndex(ipa, walk.level as int);
    let rtte = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx);
    let new_rtte = RttEntryAt(RttAt(new_s, walk.rtt_addr), entry_idx);
    let new_data_granule = GranuleAt(new_s, data);
    let old_data_granule = GranuleAt(old_s, data);
    let new_rd_granule = GranuleAt(new_s, rd);
    let old_rd_granule = GranuleAt(old_s, rd);
    let new_ipa_granule = GranuleAt(new_s, ipa);
    let old_ipa_granule = GranuleAt(old_s, ipa);

    (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDevMem(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (old_data_granule.state != DATA ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm.feat_lpa2 == FEATURE_FALSE && data >= pow2(48) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (old_rd_granule.state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, realm, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level as int > RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT))
    && (rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    && (result.is_Ok() ==> (new_data_granule.state == DATA && new_rtte.state == ASSIGNED && new_rtte.addr == data && new_rtte.attr_unprot == MEMATTR_CACHEABLE && new_rtte.sh == SHAREABILITY_INNER))
    && (result.is_Ok() ==> (new_data_granule.state == DATA && new_rtte.state == ASSIGNED && new_rtte.addr == data && new_rtte.attr_unprot == MEMATTR_CACHEABLE && new_rtte.sh == SHAREABILITY_INNER))
}
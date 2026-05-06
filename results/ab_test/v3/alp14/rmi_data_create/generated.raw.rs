```verus
pub open spec fn RMI_DATA_CREATE_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    data: Address,
    ipa: Address,
    src: Address,
    flags: RmiDataFlags,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    
    // Failure conditions (ordered by precedence)
    let src_align_fail = !AddrIsGranuleAligned(old_s, src) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let src_pas_fail = !GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let data_align_fail = !AddrIsGranuleAligned(old_s, data) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let data_bound_fail = !PaIsDelegableDram(old_s, data) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let data_state_fail = GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let data_bound2_fail = ((realm_pre.feat_lpa2 == FEATURE_FALSE) && (data as int >= (1 << 48))) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let rd_align_fail = !AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(old_s, rd) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let ipa_align_fail = !AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let ipa_bound_fail = !AddrIsProtected(old_s, ipa, realm_pre) ==> ResultEqual(ResultErr(RMI_ERROR_INPUT), RMI_ERROR_INPUT);
    let realm_state_fail = realm_pre.state != REALM_NEW ==> ResultEqual(ResultErr(RMI_ERROR_REALM), RMI_ERROR_REALM);
    let rtt_walk_fail = walk.level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(ResultErr(RMI_ERROR_RTT), RMI_ERROR_RTT);
    let rtte_state_fail = walk.rtte.state != UNASSIGNED ==> ResultEqual(ResultErr(RMI_ERROR_RTT), RMI_ERROR_RTT);
    
    // Success conditions
    let success_data_state = (GranuleAt(old_s, src).state == DELEGATED) &&
                            (GranuleAt(old_s, data).state == DELEGATED) &&
                            (GranuleAt(old_s, rd).state == RD) &&
                            (realm_pre.state == REALM_NEW) &&
                            (AddrIsGranuleAligned(old_s, src)) &&
                            (AddrIsGranuleAligned(old_s, data)) &&
                            (AddrIsGranuleAligned(old_s, rd)) &&
                            (AddrIsGranuleAligned(old_s, ipa)) &&
                            (GranuleAccessPermitted(old_s, src, PAS_NS)) &&
                            (PaIsDelegableDram(old_s, data)) &&
                            (PaIsDelegable(old_s, rd)) &&
                            (AddrIsProtected(old_s, ipa, realm_pre)) &&
                            (walk.level == RMM_RTT_PAGE_LEVEL) &&
                            (walk.rtte.state == UNASSIGNED) &&
                            ((realm_pre.feat_lpa2 != FEATURE_FALSE) || (data as int < (1 << 48)));
    
    let success_postcond = success_data_state ==>
        (GranuleAt(new_s, data).state == DATA) &&
        (RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED) &&
        (RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == RAM) &&
        (RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == data) &&
        (RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_prot == MEMATTR_CACHEABLE) &&
        (RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).sh == SHAREABILITY_INNER) &&
        (RealmAt(new_s, rd).measurements[0] == RimExtendData(old_s, realm_pre, ipa, data, flags));
    
    success_postcond && src_align_fail && src_pas_fail && data_align_fail && data_bound_fail && 
    data_state_fail && data_bound2_fail && rd_align_fail && rd_bound_fail && rd_state_fail && 
    ipa_align_fail && ipa_bound_fail && realm_state_fail && rtt_walk_fail && rtte_state_fail
}
```
pub open spec fn rmi_vsmmu_map_spec(rd: Address, vsmmu_ptr: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);
    let walk = RttWalk(old_s, realm, ipa, level as int, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let da = ImplFeatures(old_s).feat_da == FEATURE_TRUE;
    let rd_ok = AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD;
    let vsmmu_ok = AddrIsGranuleAligned(old_s, vsmmu_ptr) && PaIsDelegable(old_s, vsmmu_ptr) && GranuleAt(old_s, vsmmu_ptr).state == VSMMU;
    let input_fail = !rd_ok
        || (rd_ok && (!RttLevelIsValid(old_s, realm, level as int) || level < 2 || realm.state != REALM_NEW))
        || (da && !vsmmu_ok)
        || !AddrIsRttLevelAligned(old_s, ipa, level as int)
        || (vsmmu_ok && ipa < vsmmu.reg_base);
    let rtt_fail = rd_ok && (
        walk.level < level as int
        || walk.rtte.state != UNASSIGNED
        || walk.rtte.ripas != EMPTY
        || (vsmmu_ok && (ipa as int) + RttLevelSize(old_s, walk.level) - 1 >= (vsmmu.reg_top as int))
    );
    (!da ==> result.is_Err())
    && (input_fail ==> result.is_Err())
    && (rtt_fail ==> result.is_Err())
    && (result.is_Err() ==> (ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) || ResultEqual(result, RMI_ERROR_INPUT) || ResultEqual(result, RMI_ERROR_RTT(walk.level))))
    && (ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> !da)
    && (ResultEqual(result, RMI_ERROR_INPUT) ==> input_fail)
    && ((result.is_Err() && !ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) && !ResultEqual(result, RMI_ERROR_INPUT)) ==> (rtt_fail && ResultEqual(result, RMI_ERROR_RTT(walk.level))))
    && ((da && !input_fail && !rtt_fail) ==> result.is_Ok())
    && (result.is_Ok() ==> (
        RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED_VSMMU
        && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == vsmmu_ptr
    ))
    && (result.is_Err() ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx) == RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx))
}

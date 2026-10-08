pub open spec fn rmi_vsmmu_map_spec(rd: Address, vsmmu_ptr: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);
    let walk = RttWalk_(old_s, rd, ipa, level as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level as int);
    let rtte = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx);
    let rtte_new = RttEntryAt(RttAt(new_s, walk.rtt_addr), entry_idx);
    (!ImplFeatures(CurrentRealm(old_s)).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (level < 2 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, ipa, pow2(level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa < vsmmu.reg_base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level ==> ResultEqual(result, RMI_ERROR_RTT))
    && (rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    && (rtte.ripas != EMPTY ==> ResultEqual(result, RMI_ERROR_RTT))
    && (ipa + pow2(walk.level as int) >= vsmmu.reg_top ==> ResultEqual(result, RMI_ERROR_RTT))
    && (result.is_Ok() ==> rtte_new.state == ASSIGNED_VSMMU)
    && (result.is_Ok() ==> rtte_new.addr == vsmmu_ptr)
}
```verus
pub open spec fn RMI_VSMMU_MAP_spec(old_s: S, new_s: S, rd: Address, vsmmu_ptr: Address, ipa: Address, level: int, result: RmiCommandReturnCode) -> bool {
    let realm = RealmAt(old_s, rd);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    
    (
        // Failure: da_supp
        (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
        // Failure: rd_align
        (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: rd_bound
        (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: rd_state
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: level_bound
        ((!RttLevelIsValid(old_s, realm, level) || level < 2) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: realm_state
        (realm.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vsmmu_align
        (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vsmmu_bound
        (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: vsmmu_state
        (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: ipa_align
        (!AddrIsRttLevelAligned(old_s, ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: ipa_bound
        (UInt(ipa) < UInt(vsmmu.reg_base) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Failure: rtt_walk
        (walk.level < level ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
        // Failure: rtte_state
        (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
        // Failure: rtte_ripas
        (walk.rtte.ripas != EMPTY ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
        // Failure: rtte_bound
        ((UInt(ipa) + (RttLevelSize(old_s, walk.level) - 1) >= UInt(vsmmu.reg_top)) ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
        // Success: all preconditions pass
        (
            ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && RttLevelIsValid(old_s, realm, level)
            && level >= 2
            && realm.state == REALM_NEW
            && AddrIsGranuleAligned(old_s, vsmmu_ptr)
            && PaIsDelegable(old_s, vsmmu_ptr)
            && GranuleAt(old_s, vsmmu_ptr).state == VSMMU
            && AddrIsRttLevelAligned(old_s, ipa, level)
            && UInt(ipa) >= UInt(vsmmu.reg_base)
            && walk.level >= level
            && walk.rtte.state == UNASSIGNED
            && walk.rtte.ripas == EMPTY
            && UInt(ipa) + (RttLevelSize(old_s, walk.level) - 1) < UInt(vsmmu.reg_top)
            ==> (
                result.is_Ok()
                && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED_VSMMU
                && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == vsmmu_ptr
            )
        )
    )
}
```
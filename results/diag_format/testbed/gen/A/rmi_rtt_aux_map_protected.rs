pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, index as int);
    let pri = walk_pri.rtte;
    let aux = walk_aux.rtte;
    let rd_ok = AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD;
    let index_ok = realm.rtt_tree_per_plane == FEATURE_TRUE
        && index as int != RMM_RTT_TREE_PRIMARY
        && index <= realm.num_aux_planes;
    let input_ok = rd_ok
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, realm)
        && index_ok;
    let pri_state_bad = !(pri.state == ASSIGNED || pri.state == ASSIGNED_DEV || pri.state == ASSIGNED_VSMMU);
    let pri_ram_bad = pri.state == ASSIGNED && pri.ripas != RAM;
    let pri_dev_bad = pri.state == ASSIGNED_DEV && pri.ripas != DEV;
    let pri_fail = pri_state_bad || pri_ram_bad || pri_dev_bad;
    let aux_destroyed = aux.state == AUX_DESTROYED;
    let aux_level_bad = walk_aux.level < walk_pri.level;
    let aux_fail = aux_destroyed || aux_level_bad;
    let new_realm = RealmAt(new_s, rd);
    let new_aux = RttWalk(new_s, new_realm, ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte;
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_ok && !AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_ok && !index_ok ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((input_ok && (pri_fail || aux_fail)) ==> result.is_Err())
    && ((input_ok && pri_fail) ==> (
            (ResultEqual(result, RMI_ERROR_RTT(walk_pri.level))
                && state == RttEntryStateToRmi(old_s, pri.state)
                && ripas == RipasToRmi(old_s, pri.ripas))
            || (aux_fail && ResultEqual(result, RMI_ERROR_RTT_AUX(walk_aux.level)))))
    && ((input_ok && aux_fail) ==> (
            (ResultEqual(result, RMI_ERROR_RTT_AUX(walk_aux.level))
                && state == RttEntryStateToRmi(old_s, aux.state)
                && ripas == RipasToRmi(old_s, pri.ripas))
            || (pri_fail && ResultEqual(result, RMI_ERROR_RTT(walk_pri.level)))))
    && (result.is_Err() ==> new_s == old_s)
    && ((input_ok && !pri_fail && !aux_fail) ==> (
            result.is_Ok()
            && new_aux.state == ASSIGNED
            && RttMemAttrEqual(new_aux, pri, RTT_PROTECTED)
            && new_aux.sh == pri.sh
            && (new_aux.addr as int) == (pri.addr as int)
                + ((ipa as int - AlignDownToRttLevel(old_s, ipa, walk_pri.level) as int)
                    / RttLevelSize(old_s, walk_aux.level))
                    * RttLevelSize(old_s, walk_aux.level)))
}

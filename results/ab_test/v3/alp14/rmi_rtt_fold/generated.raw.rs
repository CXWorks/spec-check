pub open spec fn rmi_rtt_fold_spec(result: RmiCommandReturnCode, rtt: Address, rd: Address, ipa: Address, level: int, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let fold_pre = RttFold(RttAt(old_s, walk.rtte.addr));
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (UInt(ipa) >= (1 << realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> result.is_Error() && result.get_Error_level() == walk.level)
    && (walk.rtte.state != TABLE ==> result.is_Error() && result.get_Error_level() == walk.level)
    && (!RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) ==> result.is_Error() && result.get_Error_level() == level)
    && (AddrIsAuxRef(ipa, realm) ==> result.is_Error() && result.get_Error_level() == walk.level)
    && (
        (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
         && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level)
         && AddrIsRttLevelAligned(ipa, level - 1) && UInt(ipa) < (1 << realm.ipa_width)
         && walk.level >= level - 1 && walk.rtte.state == TABLE
         && RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr))
         && !AddrIsAuxRef(ipa, realm))
        ==> (result.is_Ok()
             && rtt == walk.rtte.addr
             && RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx).state == fold_pre.state
             && (fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS ==> RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx).addr == fold_pre.addr)
             && (fold_pre.state == ASSIGNED ==> (RttMemAttrEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, RTT_PROTECTED) && RttS2APEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, S2AP_INDIRECT)))
             && (fold_pre.state == ASSIGNED_NS ==> (RttMemAttrEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, RTT_UNPROTECTED) && RttS2APEqual(RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx), fold_pre, realm.rtt_s2ap_encoding)))
             && (AddrIsProtected(ipa, realm) ==> RttEntryAt(old_s, RttAt(old_s, walk.rtte.addr), entry_idx).ripas == fold_pre.ripas)
             && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED)
    )
}
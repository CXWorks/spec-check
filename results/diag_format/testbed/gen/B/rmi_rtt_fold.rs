pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
    let entry_idx = RttEntryIndex(old_s, ipa, (walk.level) as int);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (walk.rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (!RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) ==> ResultEqual(result, RMI_ERROR_RTT(level)))
    && (AddrIsAuxRef(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (ResultEqual(result, RMI_SUCCESS) ==> (walk.rtte.state == fold_pre.state))
    && (ResultEqual(result, RMI_SUCCESS) ==> ((fold_pre.state != RmmRttEntryState::UNASSIGNED && fold_pre.state != RmmRttEntryState::UNASSIGNED_NS) ==> (walk.rtte.addr == fold_pre.addr)))
    && (ResultEqual(result, RMI_SUCCESS) ==> (fold_pre.state == RmmRttEntryState::ASSIGNED ==> (RttMemAttrEqual(old_s, walk.rtte, fold_pre, RmmRttMemAttr::RTT_PROTECTED) && RttS2APEqual(old_s, walk.rtte, fold_pre, RmmRttS2APEncoding::S2AP_INDIRECT))))
    && (ResultEqual(result, RMI_SUCCESS) ==> (fold_pre.state == RmmRttEntryState::ASSIGNED_NS ==> (RttMemAttrEqual(old_s, walk.rtte, fold_pre, RmmRttMemAttr::RTT_UNPROTECTED) && RttS2APEqual(old_s, walk.rtte, fold_pre, realm.rtt_s2ap_encoding))))
    && (ResultEqual(result, RMI_SUCCESS) ==> (AddrIsProtected(old_s, ipa, realm) ==> (walk.rtte.ripas == fold_pre.ripas)))
    && (ResultEqual(result, RMI_SUCCESS) ==> (rtt == FoldedRttAddress(old_s, rd, ipa, level)))
    && (ResultEqual(result, RMI_SUCCESS) ==> (GranuleAt(old_s, rtt).state == RmmGranuleState::DELEGATED))
}
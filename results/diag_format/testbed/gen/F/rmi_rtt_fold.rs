pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
    // Failure: rd not aligned to granule size
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: rd not delegable
    (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: Granule at rd not in RD state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: level not valid or is starting level
    (!RttLevelIsValid(old_s, RealmAt(old_s, rd), level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    (RttLevelIsStarting(old_s, RealmAt(old_s, rd), level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: ipa not aligned to entry size at level-1
    (!AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: ipa outside IPA space
    (ipa >= pow2(old_s, RealmAt(old_s, rd).ipa_width as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure: RTT walk stops before level-1
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     walk.level != (level - 1) as int ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    // Failure: RTT entry not in TABLE state
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     RmmRttEntryState::TABLE != RttEntryAt(RttAt(old_s, walk.rtt_addr), RttEntryIndex(old_s, ipa, walk.level)).state ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    // Failure: Child RTT not homogeneous
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     !RttIsHomogeneous(old_s, RttAt(old_s, walk.rtt_addr)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    // Failure: ipa referenced by auxiliary RTT
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))

    // Success: result is Ok
    (result.is_Ok())
    // Success: rtt is the address of the folded RTT
    (rtt == walk.rtt_addr)
    // Success: parent RTT entry state set to folded entries' state
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
     let folded_state = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx).state;
     GranuleAt(new_s, walk.rtt_addr).state == folded_state)
    // Success: parent RTT entry output address set to folded entries' output address (if not UNASSIGNED/UNASSIGNED_NS)
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
     let folded_entry = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx);
     if folded_entry.state != UNASSIGNED && folded_entry.state != UNASSIGNED_NS {
         GranuleAt(new_s, walk.rtt_addr).addr == folded_entry.addr
     } else {
         true
     })
    // Success: parent RTT entry memory attributes set (interpreted for Protected/Unprotected)
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
     let folded_entry = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx);
     if folded_entry.state == ASSIGNED {
         let folded_attr = folded_entry.attr_prot;
         GranuleAt(new_s, walk.rtt_addr).attr_prot == folded_attr
     } else if folded_entry.state == ASSIGNED_NS {
         let folded_attr = folded_entry.attr_unprot;
         GranuleAt(new_s, walk.rtt_addr).attr_unprot == folded_attr
     } else {
         true
     })
    // Success: parent RTT entry stage 2 access permissions set (interpreted with indirect encoding or realm encoding)
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
     let folded_entry = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx);
     if folded_entry.state == ASSIGNED {
         GranuleAt(new_s, walk.rtt_addr).s2ap_indirect.base_index == folded_entry.s2ap_indirect.base_index
     } else if folded_entry.state == ASSIGNED_NS {
         GranuleAt(new_s, walk.rtt_addr).s2ap_indirect.base_index == folded_entry.s2ap_indirect.base_index
     } else {
         true
     })
    // Success: parent RTT entry RIPAS set (if ipa is Protected IPA)
    (let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
     let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
     let folded_entry = RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx);
     if GranuleAccessPermitted(old_s, ipa, PAS_REALM) {
         GranuleAt(new_s, walk.rtt_addr).ripas == folded_entry.ripas
     } else {
         true
     })
    // Success: Granule that held folded RTT is in DELEGATED state
    (GranuleAt(new_s, walk.rtt_addr).state == DELEGATED)
}
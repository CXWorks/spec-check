pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, ipa, (level - 1) as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let granule_at_rd = GranuleAt(old_s, rd);
    let granule_at_rtt = GranuleAt(old_s, rtt);
    let rtte = RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx);
    let is_protected = AddrIsProtected(old_s, ipa, realm);
    let is_valid_level = RttLevelIsValid(old_s, realm, level) && level != old_s.RealmParams.rtt_level_start;
    let is_ipa_aligned = AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int);
    let is_ipa_in_range = AddrIsWithin(old_s, ipa, 0, pow2(old_s.RealmParams.ipa_width as int) as Address);
    let is_rd_delegable = PaIsDelegable(old_s, rd);
    let is_rd_rd_state = granule_at_rd.state == RD;
    let is_walk_level_valid = walk.level >= (level - 1) as int;
    let is_walk_entry_table = rtte.state == TABLE;
    let is_target_rtt_live = RttIsLive(old_s, walk.rtt_addr);
    let is_ipa_aux = RttsAllProtectedEntriesRipas(old_s, walk.rtt_addr, walk.level, ipa, rtte.ripas);
    let is_rtt_unassigned = rtte.state == UNASSIGNED;
    let is_rtt_unassigned_ns = rtte.state == UNASSIGNED_NS;
    let is_rtt_ripas_destroyed = rtte.ripas == DESTROYED;
    let is_granule_delegated = granule_at_rtt.state == DELEGATED;
    let is_top_correct = top == walk_top;
    let is_rtt_correct = rtt == rtte.addr;
    (!is_valid_level ==> ResultEqual(result, RMI_ERROR_RTT))
    && (!is_ipa_aligned ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!is_ipa_in_range ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!is_rd_delegable ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!is_rd_rd_state ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!is_walk_level_valid ==> ResultEqual(result, RMI_ERROR_RTT))
    && (!is_walk_entry_table ==> ResultEqual(result, RMI_ERROR_RTT))
    && (is_walk_entry_table && is_target_rtt_live ==> ResultEqual(result, RMI_ERROR_RTT))
    && (is_walk_entry_table && is_ipa_aux ==> ResultEqual(result, RMI_ERROR_RTT))
    && (result.is_Ok() ==> (is_rtt_unassigned || is_rtt_unassigned_ns))
    && (result.is_Ok() ==> is_rtt_ripas_destroyed)
    && (result.is_Ok() ==> is_granule_delegated)
    && (result.is_Ok() ==> is_top_correct)
    && (result.is_Ok() ==> is_rtt_correct)
}
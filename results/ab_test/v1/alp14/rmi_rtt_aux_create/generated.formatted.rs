pub open spec fn RMI_RTT_AUX_CREATE_spec(s: S, rd: Address, rtt: Address, ipa: Address, level: int, index: u64) -> (result: Result<(), RmiStatusCode>, s_prime: S)
{
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let unfold = RttWalk(s, realm, ipa, level - 1, index).rtte;
    
    // Failure conditions
    if !AddrIsGranuleAligned(rd) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(rd) {
        (Err(RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, rd).state != RD {
        (Err(RMI_ERROR_INPUT), s)
    } else if !RttLevelIsValid(s, realm, level) || RttLevelIsStarting(s, realm, level) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !AddrIsRttLevelAligned(ipa, level - 1) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !AddrIsProtected(s, ipa, realm) {
        (Err(RMI_ERROR_INPUT), s)
    } else if realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes {
        (Err(RMI_ERROR_INPUT), s)
    } else if !AddrIsGranuleAligned(rtt) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !PaIsDelegableDram(rtt) {
        (Err(RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, rtt).state != DELEGATED {
        (Err(RMI_ERROR_INPUT), s)
    } else if realm.feat_lpa2 == FEATURE_FALSE && UInt(rtt) >= 0x1000000000000 {
        (Err(RMI_ERROR_INPUT), s)
    } else if walk.level < level - 1 {
        (Err(RMI_ERROR_RTT_AUX), s)
    } else if walk.rtte.state == TABLE {
        (Err(RMI_ERROR_RTT_AUX), s)
    } else {
        // Success case
        let s_prime = s.with_granule(rtt, GranuleAt(s, rtt).with_state(RTT))
                       .with_rtt_entries(rtt, |i| 
                           RmmRttEntry {
                               state: unfold.state,
                               ripas: unfold.ripas,
                               addr: if unfold.state != UNASSIGNED then unfold.addr else 0,
                           }
                       );
        (Ok(()), s_prime)
    }
}
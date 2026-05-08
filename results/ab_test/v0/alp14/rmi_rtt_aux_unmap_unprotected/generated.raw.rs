```verus
pub open spec fn RMI_RTT_AUX_UNMAP_UNPROTECTED_spec(s: S, rd: Address, ipa: Address, index: u64, result: Result<Address, RmiStatusCode>) -> bool {
  let realm = RealmAt(s, rd);
  let walk = RttWalk(s, realm, ipa, realm.rtt_level_start, index);
  let entry_idx = RttEntryIndex(s, ipa, walk.level);
  let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);

  (
    // Failure conditions
    (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(s, rd).state == RD &&
     AddrIsRttLevelAligned(ipa, realm.rtt_level_start) &&
     UInt(ipa) < (2 ^ realm.ipa_width) && !AddrIsProtected(ipa, realm) &&
     (realm.rtt_tree_per_plane == FEATURE_TRUE && index != RMM_RTT_TREE_PRIMARY && index <= realm.num_aux_planes)) ==>
    (
      // Success conditions
      result.is_Ok() &&
      RttEntryAt(s, RttAt(s, walk.rtt_addr), entry_idx).state == UNASSIGNED_NS &&
      result.get_Ok_0() == walk_top
    )
  ) && (
    // Failure on rd alignment
    (!AddrIsGranuleAligned(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)
  ) && (
    // Failure on rd bound
    (!PaIsDelegable(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)
  ) && (
    // Failure on rd state
    (GranuleAt(s, rd).state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT)
  ) && (
    // Failure on ipa alignment
    (!AddrIsRttLevelAligned(ipa, realm.rtt_level_start)) ==> ResultEqual(result, RMI_ERROR_INPUT)
  ) && (
    // Failure on ipa bound
    (UInt(ipa) >= (2 ^ realm.ipa_width) || AddrIsProtected(ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)
  ) && (
    // Failure on index bound
    (realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)
  )
}
```
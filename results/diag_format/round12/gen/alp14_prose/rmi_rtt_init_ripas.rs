pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (GranuleAt(old_s, rd).state != GRANULE_STATE_DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRANULE_STATE_IN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_protected_ipa(old_s, RealmAt(old_s, rd), (top + GRANULE_SIZE) - GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) % (RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entry_size) != 0 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((top + GRANULE_SIZE) % GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).top == top ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> out_top == RttSkipEntriesIfNotState(old_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED))
  && ((!(GranuleAt(old_s, rd).state != GRANULE_STATE_DELEGATED) &&
       GranuleAt(old_s, rd).state == GRANULE_STATE_IN_RD &&
       RealmAt(old_s, rd).state == REALM_NEW &&
       !(top <= base) &&
       !(is_protected_ipa(old_s, RealmAt(old_s, rd), (top + GRANULE_SIZE) - GRANULE_SIZE)) &&
       !((base) % (RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entry_size) != 0) &&
       !(RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).state != UNASSIGNED) &&
       !((top + GRANULE_SIZE) % GRANULE_SIZE != 0) &&
       !(RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).top == top)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
}
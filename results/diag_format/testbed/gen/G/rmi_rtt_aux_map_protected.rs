pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, ipa) ==> result.is_Ok())
  && (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || !(index == RMM_RTT_TREE_PRIMARY) && !(index > RealmAt(old_s, rd).num_aux_planes) ==> result.is_Ok())
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED || RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED_DEV || RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED_VSMMU ==> result.is_Ok())
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RAM ==> result.is_Ok())
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED_DEV && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == DEV ==> result.is_Ok())
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state == AUX_DESTROYED ==> result.is_err())
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state == ASSIGNED)
  && ((!(AddrIsGranuleAligned(old_s, rd)) ||
       !(AddrIsGranuleAligned(old_s, ipa)) ||
       (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_TRUE &&
        !(index == RMM_RTT_TREE_PRIMARY) &&
        !(index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED || RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED_DEV || RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED_VSMMU))
     ==> result.is_Ok())
  && (result.is_Err()
     ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state)
  && (result.is_Err()
     ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas)
  && (result.is_Err()
     ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state)
  && (result.is_Err()
     ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).ripas == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas)
}
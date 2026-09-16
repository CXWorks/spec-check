pub open spec fn rmi_rtt_aux_destroy_spec(rd: Address, ipa: Address, level: Int64, index: UInt64, result: Result<(Address, Address), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> result.status == RMI_ERROR_INPUT)
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> result.status == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1 ==> result.status == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != RTTE_TABLE ==> (result.status == RMI_ERROR_RTT_AUX && result.data.level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level && top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level,ipa))))
  && (RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)) ==> (result.status == RMI_ERROR_RTT_AUX && result.data.level.level == level && top == ipa))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RIPAS_DESTROYED)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level,ipa))
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RTTE_AUX_DESTROYED)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state == GRAN_DELEGATED)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != RTTE_TABLE) &&
       !(RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.ripas)
  && (result.is_Err()
    ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).level,ipa))
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state)
}
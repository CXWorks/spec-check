pub open spec fn rmi_rtt_aux_fold_spec(rd: Address, ipa: Address, level: Int64, index: UInt64, result: Result<(Address, RttWalkResult), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> result.status == RMI_ERROR_INPUT)
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> result.status == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level < level - 1 ==> (result.status == RMI_ERROR_RTT_AUX && result.1.level == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state != RTTE_TABLE ==> (result.status == RMI_ERROR_RTT_AUX && result.1.level == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).level))
  && (!RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)) ==> (result.status == RMI_ERROR_RTT_AUX && result.1.level == level))
  && (result.is_Ok() ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state)
  && (result.is_Ok() && RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state != RTTE_VOID && RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state != RTTE_UNMAPPED_NS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).addr)
  && (result.is_Ok() && RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state == RTTE_DATA ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)), RTT_PROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)), S2AP_INDIRECT)))
  && (result.is_Ok() && RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).state == RTTE_MAPPED_NS ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)), RTT_UNPROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)), RealmAt(new_s, rd).rtt_s2ap_encoding)))
  && (result.is_Ok() && AddrIsProtected(new_s, ipa, RealmAt(new_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr)).ripas)
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
       RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)))
    ==> result.status == RMI_SUCCESS)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.ripas)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index as int).rtte.addr).state)
}
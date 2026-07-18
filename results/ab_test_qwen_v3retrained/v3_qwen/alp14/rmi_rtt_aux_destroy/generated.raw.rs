pub open spec fn rmi_rtt_aux_destroy_spec(rd: Address, ipa: Address, level: Int64, index: UInt64, result: Result<RmiCommandReturnCode, RmiStatusCode>, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).level as int)) && (AddrEqual(result.get_Ok_0(new_s).top, RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).level,ipa)))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.state != TABLE ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).level as int)) && (AddrEqual(result.get_Ok_0(new_s).top, RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).level,ipa)))))
  && (RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.addr)) ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(level as int)) && (AddrEqual(result.get_Ok_0(new_s).top, ipa))))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtte.state == AUX_DESTROYED)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> AddrEqual(result.get_Ok_0(new_s).rtt, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtte.addr))
  && (result.is_Ok() ==> AddrEqual(result.get_Ok_0(new_s).top, RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).level,ipa)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.state != TABLE) &&
       !(RttIsLive(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.addr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.ripas)
  && (result.is_Err()
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,index).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.addr).state)
  && (result.is_Err()
    ==> AddrEqual(result.get_Ok_0(new_s).rtt, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtte.addr))
  && (result.is_Err()
    ==> AddrEqual(result.get_Ok_0(new_s).top, RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,index).level,ipa)))
}
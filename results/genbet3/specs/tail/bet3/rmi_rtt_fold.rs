pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result == RMI_ERROR_INPUT)
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> result == RMI_ERROR_INPUT)
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> result == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1 ==> (result == RMI_ERROR_RTT && (result.get_Err_0().level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level))))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_TABLE ==> (result == RMI_ERROR_RTT && (result.get_Err_0().level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level))))
  && (!RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)) ==> (result == RMI_ERROR_RTT && (result.get_Err_0().level.level == level)))
  && (AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)) ==> (result == RMI_ERROR_RTT && (result.get_Err_0().level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level))))
  && (result == RMI_SUCCESS ==> rtt == RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result == RMI_SUCCESS ==> result.get_Err_0().status == RMI_SUCCESS)
  && (result == RMI_SUCCESS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state)
  && (result == RMI_SUCCESS && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state != RTTE_VOID && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state != RTTE_UNMAPPED_NS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).addr)
  && (result == RMI_SUCCESS && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state == RTTE_DATA ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)), RTT_PROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)), S2AP_INDIRECT)))
  && (result == RMI_SUCCESS && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).state == RTTE_MAPPED_NS ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)), RTT_UNPROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte, RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)),RealmAt(new_s, rd).rtt_s2ap_encoding)))
  && (result == RMI_SUCCESS && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttFold(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)).ripas)
  && (result == RMI_SUCCESS ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GRAN_DELEGATED)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_TABLE) &&
       RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)) &&
       !(AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd))))
    ==> result == RMI_SUCCESS)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (result != RMI_SUCCESS
    ==> GranuleAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == GranuleAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.prot)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_encoding == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_encoding)
}
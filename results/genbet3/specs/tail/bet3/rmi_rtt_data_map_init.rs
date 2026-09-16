pub open spec fn rmi_rtt_data_map_init_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, src) ==> result == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, src) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, data) ==> result == RMI_ERROR_INPUT)
  && (!PaIsPopulatedConventional(old_s, data) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, data) ==> (result == RMI_ERROR_TRACKING && RttWalk(new_s, RealmAt(new_s, rd), data,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level_addr.level == TrackingToRmiResult(new_s, data) && RttWalk(new_s, RealmAt(new_s, rd), data,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level_addr.addr == AddrShiftRmiGranule(new_s, data)))
  && (GranuleAt(old_s, data).state != GRAN_DELEGATED ==> result == RMI_ERROR_INPUT)
  && ((data) >= pow2(RealmAt(old_s, rd).static_.max_oaddr_width as nat) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, ipa) ==> result == RMI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> result == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result == RMI_ERROR_REALM(0))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL ==> (result == RMI_ERROR_RTT && RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID ==> (result == RMI_ERROR_RTT && RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level.level == RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (result == RMI_SUCCESS ==> GranuleAt(new_s, data).state == GRAN_DATA)
  && (result == RMI_SUCCESS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_RAM)
  && (result == RMI_SUCCESS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == data)
  && (result == RMI_SUCCESS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == MEMATTR_CACHEABLE)
  && (result == RMI_SUCCESS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == SHAREABILITY_INNER)
  && (result == RMI_SUCCESS ==> RealmAt(new_s, rd).rim == RimExtendData(new_s, RealmAt(new_s, rd), ipa, data, flags))
  && (result == RMI_SUCCESS ==> result == RMI_SUCCESS)
  && (result == RMI_SUCCESS ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_DATA)
  && ((AddrIsRmiGranuleAligned(old_s, src) &&
       NonSecureAccessPermitted(old_s, src) &&
       AddrIsRmiGranuleAligned(old_s, data) &&
       PaIsPopulatedConventional(old_s, data) &&
       PaIsTrackedFine(old_s, data) &&
       !(GranuleAt(old_s, data).state != GRAN_DELEGATED) &&
       !((data) >= pow2(RealmAt(old_s, rd).static_.max_oaddr_width as nat)) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID))
    ==> result == RMI_SUCCESS)
  && (result != RMI_SUCCESS
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh)
  && (result != RMI_SUCCESS
    ==> RealmAt(new_s, rd).rim == RealmAt(old_s, rd).rim)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas &&
       result != RMI_SUCCESS
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_RAM)
}
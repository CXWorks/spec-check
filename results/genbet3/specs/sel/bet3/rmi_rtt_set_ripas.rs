pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, rec_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rec_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rec_ptr).state != GRAN_REC ==> result.status == RMI_ERROR_INPUT)
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> result.status == RMI_ERROR_INPUT)
  && (RecAt(old_s, rec_ptr).owner != rd ==> result.status == RMI_ERROR_REC)
  && ((top) <= (base) ==> result.status == RMI_ERROR_INPUT)
  && (base != RecAt(old_s, rec_ptr).ripas_addr ==> result.status == RMI_ERROR_INPUT)
  && ((top) > (RecAt(old_s, rec_ptr).ripas_top) ==> result.status == RMI_ERROR_INPUT)
  && ((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value) ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && (((top) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(old_s, rec_ptr).ripas_value == RIPAS_RAM) &&(RecAt(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value) ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)) ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_RAM && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (result.is_Ok() && (RealmAt(old_s, rd).ats && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr != 0 && !(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr % 64 == 0)) ==> result.status == RMI_ERROR_DPT)
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(new_s, rec_ptr).ripas_value == RIPAS_RAM) &&(RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)),RecAt(new_s, rec_ptr).ripas_value))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).ripas_addr == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(new_s, rec_ptr).ripas_value == RIPAS_RAM) &&(RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))))
  && (result.is_Ok() ==> out_top == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(new_s, rec_ptr).ripas_value == RIPAS_RAM) &&(RecAt(new_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))))
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, rec_ptr) &&
       PaIsTracked(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != GRAN_REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != RecAt(old_s, rec_ptr).ripas_addr) &&
       !((top) > (RecAt(old_s, rec_ptr).ripas_top)) &&
       !((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !(((top) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(old_s, rec_ptr).ripas_value == RIPAS_RAM) &&(RecAt(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))) && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value)) &&
       !(AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
}
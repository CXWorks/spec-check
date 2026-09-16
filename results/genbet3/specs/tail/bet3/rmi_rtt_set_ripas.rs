pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != GRAN_REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (RecAt(old_s, rec_ptr).ripas_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) && RecAt(old_s, rec_ptr).ripas_value != RecAt(old_s, rec_ptr).ripas_value) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(old_s, rec_ptr).ripas_value == RIPAS_RAM) &&(RecAt(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED))) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))))
  && (AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))))
  && (RecAt(old_s, rec_ptr).ripas_value == RIPAS_RAM && (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))))
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
       !((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) && RecAt(old_s, rec_ptr).ripas_value != RecAt(old_s, rec_ptr).ripas_value)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) == (RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top,(RecAt(old_s, rec_ptr).ripas_value == RIPAS_RAM) &&(RecAt(old_s, rec_ptr).ripas_destroyed!=CHANGE_DESTROYED)))) &&
       !(AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)))) &&
       !(RecAt(old_s, rec_ptr).ripas_value == RIPAS_RAM && (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
}
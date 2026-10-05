pub open spec fn rmi_data_destroy_spec(rd: RealmDescriptor, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> GranuleAt(new_s, data).state == DELEGATED)
  && (result.is_Ok() ==> GranuleAt(new_s, data).state == DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_entries[RttIndex(new_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].state == UNASSIGNED)
  && (result.is_Ok() && RealmAt(old_s, rd).rtt_entries[RttIndex(old_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].ripas == RIPAS_RAM ==> RealmAt(new_s, rd).rtt_entries[RttIndex(new_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].ripas == RIPAS_DESTROYED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_entries[RttIndex(new_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].state == UNASSIGNED)
  && ((!(rd.align_to(GranuleSize) == true) ==> ResultEqual(result, RMI_ERROR_INPUT))
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtt_entries[RttIndex(new_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].ripas == RealmAt(old_s, rd).rtt_entries[RttIndex(old_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].ripas)
  && ((!(ipa.align_to(GranuleSize) == true) ==> ResultEqual(result, RMI_ERROR_INPUT))
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Ok() ==> top > ipa)
  && (result.is_Ok() && !(ipa.align_to(GranuleSize) == true) ==> top > ipa)
  && (result.is_Ok() && !(rd.align_to(GranuleSize) == true) ==> top > ipa)
  && (result.is_Ok() && !(rd.is_delegable_physical_address() == true) ==> top > ipa)
  && (result.is_Ok() && !(GranuleAt(old_s, rd).state == RD_STATE) ==> top > ipa)
  && (result.is_Ok() && !(ipa.in_protected_ipa_space(RealmAt(old_s, rd)) == true) ==> top > ipa)
  && (result.is_Ok() && RttWalk(old_s, rd, ipa).level > RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa).level as int)))
  && (result.is_Ok() && RttWalk(old_s, rd, ipa).state != ASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa).level as int)))
  && (result.is_Ok() && RttWalk(old_s, rd, ipa).is_live_in_aux_rtt(RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0)))
  && (result.is_Err() ==> RealmAt(new_s, rd).rtt_entries[RttIndex(new_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].state == RealmAt(old_s, rd).rtt_entries[RttIndex(old_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].state)
  && (result.is_Err() ==> RealmAt(new_s, rd).rtt_entries[RttIndex(new_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].ripas == RealmAt(old_s, rd).rtt_entries[RttIndex(old_s, rd, ipa as int, RMM_RTT_PAGE_LEVEL as int)].ripas)
  && (result.is_Err() ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && ((result.is_Ok() && (RttWalk(old_s, rd, ipa).level <= RMM_RTT_PAGE_LEVEL))
    ==> GranuleAt(new_s, data).state == DELEGATED)
  && (result.is_Err() && !(RttWalk(old_s, rd, ipa).level > RMM_RTT_PAGE_LEVEL) &&
       RttWalk(old_s, rd, ipa).state == ASSIGNED &&
       !(RttWalk(old_s, rd, ipa).is_live_in_aux_rtt(RealmAt(old_s, rd)))
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
  && (result.is_Ok() && RttWalk(old_s, rd, ipa).state == ASSIGNED
    ==> GranuleAt(new_s, data).state == DELEGATED)
  && (result.is_Ok() && RttWalk(old_s, rd, ipa).state == UNASSIGNED
    ==> GranuleAt(new_s, data).state == GranuleAt(old_s, data).state)
}
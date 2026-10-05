pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((!(GranuleAt(old_s, rd).state == GRANULE_STATE_RD) &&
    !(GranuleAt(old_s, rd).state == GRANULE_STATE_DELEGATED))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT)
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Ok()
    ==> GranuleAt(new_s, rd).state == GRANULE_STATE_DELEGATED)
  && (result.is_Ok()
    ==> RealmAt(new_s, rd).rtt_base[0] == RealmAt(old_s, rd).rtt_base[0])
  && (result.is_Ok()
    ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(old_s, rd).rtt_num_start)
  && ((!(GranuleAt(old_s, rd).state == GRANULE_STATE_RD) &&
       !(GranuleAt(old_s, rd).state == GRANULE_STATE_DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Ok()
    ==> RealmAt(new_s, rd).rtt_base[0] == RealmAt(old_s, rd).rtt_base[0])
  && (result.is_Ok()
    ==> RealmAt(new_s, rd).rtt_num_start == RealmAt(old_s, rd).rtt_num_start)
}
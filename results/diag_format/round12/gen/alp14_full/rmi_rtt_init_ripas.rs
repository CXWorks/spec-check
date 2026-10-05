pub open spec fn rmi_rtt_init_ripas_spec(rd: Rd, base: UInt64, top: UInt64, result: Result<(), RmiStatusCode>, out_top: UInt64, old_s: S, new_s: S) -> bool {
  (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Err() ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_NEW)
  && ((!(GranuleAt(old_s, rd).state != RD) &&
       !(top <= base) &&
       !(RealmAt(old_s, rd).state != REALM_NEW))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> out_top == 0)
}
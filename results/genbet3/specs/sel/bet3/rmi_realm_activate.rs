pub open spec fn rmi_realm_activate_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result.status == RMI_ERROR_REALM(0))
  && (RealmSroIsIncomplete(old_s, RealmAt(old_s, rd)) ==> result.status == RMI_BLOCKED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_ACTIVE)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RealmSroIsIncomplete(old_s, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
}
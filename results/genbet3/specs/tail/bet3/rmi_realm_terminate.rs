pub open spec fn rmi_realm_terminate_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (AnyRecRunning(old_s, RealmAt(old_s, rd)) ==> result.status == RMI_ERROR_REALM(0))
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_ZOMBIE)
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTrackedFine(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !(AnyRecRunning(old_s, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
}
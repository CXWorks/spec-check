pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_ZOMBIE ==> result.status == RMI_ERROR_REALM(0))
  && (RealmIsLive(old_s, rd) ==> result.status == RMI_ERROR_REALM(0))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && (result.is_Ok() ==> Rmm().dynamic.num_realms == Rmm().dynamic.num_realms - 1)
  && (result.is_Ok() ==> RttsStateEqual(RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, GRAN_DELEGATED))
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == GRAN_DELEGATED)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTrackedFine(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !(RealmAt(old_s, rd).state != REALM_ZOMBIE) &&
       !(RealmIsLive(old_s, rd)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> Rmm().dynamic.num_realms == Rmm().dynamic.num_realms)
  && (result.is_Err()
    ==> RttsStateEqual(RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int, RealmAt(new_s, rd).state))
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
}
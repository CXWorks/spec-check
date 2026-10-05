pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() ==> RttsStateEqual(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start, DELEGATED))
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == DELEGATED)
  && (result.is_Ok() ==> VmidsAreFree(new_s, RealmAt(new_s, rd).vmid))
  && (result.is_Ok() && RealmAt(old_s, rd).mec_policy == MEC_POLICY_PRIVATE ==> MecState(new_s, RealmAt(new_s, rd).mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
  && (result.is_Ok() && RealmAt(old_s, rd).mec_policy == MEC_POLICY_SHARED ==> MecMembers(new_s, RealmAt(new_s, rd).mecid) == MecMembers(old_s, RealmAt(old_s, rd).mecid) - 1)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmIsLive(old_s, rd)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> VmidsAreFree(new_s, RealmAt(new_s, rd).vmid))
  && (result.is_Err()
    ==> MecState(new_s, RealmAt(new_s, rd).mecid) == MecState(old_s, RealmAt(old_s, rd).mecid))
  && (result.is_Err()
    ==> MecMembers(new_s, RealmAt(new_s, rd).mecid) == MecMembers(old_s, RealmAt(old_s, rd).mecid))
}
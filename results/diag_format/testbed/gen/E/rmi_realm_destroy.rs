pub open spec fn rmi_realm_destroy_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() ==> RttsGranuleState(new_s, RealmAt(new_s, rd).rtt_base[0], RealmAt(new_s, rd).rtt_num_start as int))
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == DELEGATED)
  && (result.is_Ok() ==> VmidsAreValid(new_s, RealmAt(new_s, rd).vmid[0], RealmAt(new_s, rd).aux_vmid))
  && (result.is_Ok() ==> MecState(new_s, RealmAt(new_s, rd).mecid) == MEC_STATE_PRIVATE_UNASSIGNED ==> RealmAt(new_s, rd).mec_policy == MEC_POLICY_PRIVATE)
  && (result.is_Ok() ==> MecMembers(new_s, RealmAt(new_s, rd).mecid) == MecMembers(new_s, RealmAt(new_s, rd).mecid) - 1 ==> RealmAt(new_s, rd).mec_policy == MEC_POLICY_SHARED)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmAt(old_s, rd).state != REALM_SYSTEM_OFF))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).mec_policy == RealmAt(old_s, rd).mec_policy)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).mecid == RealmAt(old_s, rd).mecid)
}
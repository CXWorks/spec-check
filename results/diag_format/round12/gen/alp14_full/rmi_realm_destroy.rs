pub open spec fn rmi_realm_destroy_spec(rd: PhysicalAddress, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((rd) % granule_size(old_s) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!can_delegate(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && RealmAt(old_s, rd).live ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (result.is_Ok() ==> GranuleAt(new_s, rd).state == DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, rd).rtts[0].state == DELEGATED)
  && (result.is_Ok() && RealmAt(old_s, rd).mec_policy == MEC_POLICY_PRIVATE ==> RealmAt(new_s, rd).mec_id == MEC_STATE_PRIVATE_UNASSIGNED)
  && (result.is_Ok() && RealmAt(old_s, rd).mec_policy == MEC_POLICY_SHARED ==> RealmAt(new_s, rd).mec_id.len() == RealmAt(old_s, rd).mec_id.len() - 1)
  && ((!( (rd) % granule_size(old_s) != 0) &&
       can_delegate(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rtts[0].state == RealmAt(old_s, rd).rtts[0].state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).mec_id == RealmAt(old_s, rd).mec_id)
  && (!(result.is_Ok() && (RealmAt(old_s, rd).mec_policy == MEC_POLICY_SHARED)) ==> RealmAt(new_s, rd).mec_id == RealmAt(old_s, rd).mec_id)
}
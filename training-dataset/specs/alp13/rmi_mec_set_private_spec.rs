pub open spec fn rmi_mec_set_private_spec(mecid: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((mecid) > (ImplFeatures(old_s).max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecState(old_s, mecid) != MEC_STATE_SHARED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecMembers(old_s, mecid) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> MecState(new_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
  && ((!((mecid) > (ImplFeatures(old_s).max_mecid)) &&
       !(MecState(old_s, mecid) != MEC_STATE_SHARED) &&
       !(MecMembers(old_s, mecid) != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> MecState(new_s, mecid) == MecState(old_s, mecid))
}

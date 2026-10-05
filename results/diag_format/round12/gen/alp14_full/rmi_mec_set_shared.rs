pub open spec fn rmi_mec_set_shared_spec(mecid: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (mecid > max_mecid(old_s) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (MecState(old_s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> MecState(new_s, mecid) == MEC_STATE_SHARED)
  && ((!(mecid > max_mecid(old_s)) &&
       MecState(old_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
    ==> result.is_Ok())
  && (result.is_Err()
    ==> MecState(new_s, mecid) == MecState(old_s, mecid))
}
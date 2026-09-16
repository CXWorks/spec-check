pub open spec fn rmi_rmm_activate_spec(result: Result<(), RmiStatusCode>, rmm: RmmGlobal, old_s: S, new_s: S) -> bool {
  (rmm.dynamic.state != RMM_STATE_INIT ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (result.is_Ok() ==> result.get_Ok_0().status == RMI_SUCCESS)
  && (result.is_Ok() ==> rmm.dynamic.state == RMM_STATE_ACTIVE)
  && ((!(rmm.dynamic.state != RMM_STATE_INIT))
    ==> result.is_Ok())
}
pub open spec fn rmi_rmm_deactivate_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (RmmGlobal(old_s).dynamic.state != RMM_STATE_ACTIVE ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (AnyGranuleNotUndelegated(old_s) ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (result.is_Ok() ==> result.get_Ok_0().status == RMI_SUCCESS)
  && (result.is_Ok() ==> RmmGlobal(new_s).dynamic.state == RMM_STATE_INIT)
  && ((!(RmmGlobal(old_s).dynamic.state != RMM_STATE_ACTIVE) &&
       !(AnyGranuleNotUndelegated(old_s)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RmmGlobal(new_s).dynamic.state == RmmGlobal(old_s).dynamic.state)
}
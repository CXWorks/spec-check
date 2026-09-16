pub open spec fn rmi_rmm_deactivate_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (RmmDynamic(new_s).state != RMM_STATE_ACTIVE ==> ResultEqual(result, RMI_ERROR_GLOBAL(0)))
  && (AnyGranuleNotUndelegated(old_s) ==> ResultEqual(result, RMI_ERROR_GLOBAL(0)))
  && (result.is_Ok() ==> ResultEqual(result, RMI_SUCCESS))
  && (result.is_Ok() ==> RmmDynamic(new_s).state == RMM_STATE_INIT)
  && ((!(RmmDynamic(new_s).state != RMM_STATE_ACTIVE) &&
       !(AnyGranuleNotUndelegated(old_s)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RmmDynamic(new_s).state == RmmDynamic(old_s).state)
}
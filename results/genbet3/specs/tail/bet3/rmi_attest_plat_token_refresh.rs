pub open spec fn rmi_attest_plat_token_refresh_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().dynamic.state != RMM_STATE_ACTIVE ==> ResultEqual(result, RMI_ERROR_GLOBAL))
  && (result.is_Ok() ==> Rmm().dynamic.pat_valid == RMM_TRUE)
  && ((!(Rmm().dynamic.state != RMM_STATE_ACTIVE))
    ==> result.is_Ok())
}
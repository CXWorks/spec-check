pub open spec fn rmi_rmm_state_get_spec(result: Result<(), RmiStatusCode>, state: RmiRmmState, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> result.get_Ok_0() == RMI_SUCCESS)
  && (result.is_Ok() ==> Equal(state, Rmm()(new_s).dynamic.state))
  && ((!(result.is_Ok()))
    ==> Equal(Rmm()(new_s).dynamic.state, Rmm()(old_s).dynamic.state))
}
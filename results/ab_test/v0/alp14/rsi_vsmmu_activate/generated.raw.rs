```verus
pub open spec fn RSI_VSMMU_ACTIVATE_spec(s: S, base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address) -> bool {
  let realm = CurrentRealm(s);
  let walk = RttWalk(s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
  let vsmmu = VsmmuAt(s, walk.rtte.addr);
  
  let base_align_check = !AddrIsGranuleAligned(base);
  let top_align_check = !AddrIsGranuleAligned(top);
  let size_valid_check = UInt(top) <= UInt(base);
  let rgn_bound_check = !AddrRangeIsProtected(s, base, top, realm);
  let rtte_state_check = walk.rtte.state != ASSIGNED_VSMMU;
  
  if base_align_check || top_align_check || size_valid_check || rgn_bound_check || rtte_state_check {
    result == RSI_ERROR_INPUT
  } else {
    let ripas_condition = RttEntriesInRangeRipas(s, walk.rtte, RMM_RTT_PAGE_LEVEL, base, new_base, DEV);
    let start_condition = (base == vsmmu.reg_base && new_base != vsmmu.reg_top) ==> vsmmu.state == VSMMU_ACTIVATING;
    let complete_condition = (new_base == vsmmu.reg_top) ==> vsmmu.state == VSMMU_ACTIVE;
    
    result == RSI_SUCCESS && ripas_condition && start_condition && complete_condition
  }
}
```
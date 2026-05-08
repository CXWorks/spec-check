pub open spec fn RSI_PLANE_SYSREG_READ_spec(
    old_s: S,
    plane_idx: u64,
    addr: RsiSysregAddress,
    result: RsiCommandReturnCode,
    value_low: u64,
    value_high: u64,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    
    // Failure condition: idx_bound
    (plane_idx > realm.num_aux_planes ==> result == RSI_ERROR_INPUT) &&
    
    // Failure condition: sysreg_valid
    (!PlaneSysregValid(old_s, rec, addr, RMM_READ) ==> result == RSI_ERROR_INPUT) &&
    
    // Success condition: value_64
    ((addr.d128 == RSI_FALSE && 
      plane_idx <= realm.num_aux_planes &&
      PlaneSysregValid(old_s, rec, addr, RMM_READ)) ==>
     (result == RSI_SUCCESS &&
      (Zeros(64) :: value_low) == PlaneSysregValue(old_s, rec, plane_idx, addr))) &&
    
    // Success condition: value_128
    ((addr.d128 == RSI_TRUE &&
      plane_idx <= realm.num_aux_planes &&
      PlaneSysregValid(old_s, rec, addr, RMM_READ)) ==>
     (result == RSI_SUCCESS &&
      (value_high :: value_low) == PlaneSysregValue(old_s, rec, plane_idx, addr)))
}
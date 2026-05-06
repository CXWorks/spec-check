```verus
pub open spec fn RSI_PLANE_SYSREG_READ_spec(s: S, realm: RmmRealm, rec: RmmRec, plane_idx: u64, addr: RsiSysregAddress, result: RsiCommandReturnCode, value_low: u64, value_high: u64) -> bool {
    let failure_idx_bound = plane_idx > realm.num_aux_planes ==> result == RSI_ERROR_INPUT;
    let failure_sysreg_valid = !PlaneSysregValid(rec, addr, RMM_READ) ==> result == RSI_ERROR_INPUT;
    let success_value_64 = addr.d128 == RSI_FALSE ==> (Zeros(64) as u64 == value_low && value_high == 0 && (value_high << 64 | value_low) == PlaneSysregValue(s, rec, plane_idx, addr));
    let success_value_128 = addr.d128 == RSI_TRUE ==> ((value_high << 64 | value_low) == PlaneSysregValue(s, rec, plane_idx, addr));
    
    (failure_idx_bound && failure_sysreg_valid) || (plane_idx <= realm.num_aux_planes && PlaneSysregValid(rec, addr, RMM_READ) && (success_value_64 || success_value_128))
}
```
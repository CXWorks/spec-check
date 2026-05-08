pub open spec fn RSI_PLANE_SYSREG_READ_spec(
    old_s: S,
    new_s: S,
    plane_idx: u64,
    addr: RsiSysregAddress,
    result: RsiCommandReturnCode,
    value_low: u64,
    value_high: u64
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    
    (plane_idx > realm.num_aux_planes ==> result == RSI_ERROR_INPUT()) &&
    (!PlaneSysregValid(rec, addr, RMM_READ()) ==> result == RSI_ERROR_INPUT()) &&
    (
        (plane_idx <= realm.num_aux_planes && PlaneSysregValid(rec, addr, RMM_READ())) ==>
        (
            (addr.d128 == RSI_FALSE() ==>
                ((0u64 as int) << 64 | (value_low as int)) == (PlaneSysregValue(rec, plane_idx, addr) as int)
            ) &&
            (addr.d128 == RSI_TRUE() ==>
                ((value_high as int) << 64 | (value_low as int)) == (PlaneSysregValue(rec, plane_idx, addr) as int)
            )
        )
    ) &&
    old_s == new_s
}
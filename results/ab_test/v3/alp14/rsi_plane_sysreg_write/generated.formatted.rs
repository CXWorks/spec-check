pub open spec fn RSI_PLANE_SYSREG_WRITE_spec(
    old_s: S,
    new_s: S,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);

    // Failure condition: idx_bound
    (plane_idx > realm.num_aux_planes ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: sysreg_valid
    (!PlaneSysregValid(old_s, rec, addr, RMM_WRITE) ==> result == RSI_ERROR_INPUT)
        &&
    // Success conditions
    ((plane_idx <= realm.num_aux_planes && PlaneSysregValid(old_s, rec, addr, RMM_WRITE)) ==> (
    result == RSI_SUCCESS && PlaneSysregValue(new_s, rec, plane_idx, addr).bits_64_0() == value_low
        && (addr.d128 == RSI_TRUE ==> PlaneSysregValue(new_s, rec, plane_idx, addr).bits_128_64()
        == value_high)))
}
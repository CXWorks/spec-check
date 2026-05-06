pub open spec fn RSI_PLANE_SYSREG_READ_spec(
    s: S,
    plane_idx: u64,
    addr: RsiSysregAddress,
) -> (result: RsiCommandReturnCode, value_low: u64, value_high: u64)
{
    let realm = CurrentRealm(s);
    let rec = CurrentRec(s);
    
    if plane_idx > realm.num_aux_planes {
        (RSI_ERROR_INPUT, 0, 0)
    } else if !PlaneSysregValid(s, rec, addr, RMM_READ) {
        (RSI_ERROR_INPUT, 0, 0)
    } else {
        let sysreg_value = PlaneSysregValue(s, rec, plane_idx, addr);
        if addr.d128 == RSI_FALSE {
            // value_64: (Zeros(64) :: value_low) == sysreg_value
            let value_low = (sysreg_value & 0xFFFFFFFFFFFFFFFF) as u64;
            (RSI_OK, value_low, 0)
        } else {
            // value_128: (value_high :: value_low) == sysreg_value
            let value_low = (sysreg_value & 0xFFFFFFFFFFFFFFFF) as u64;
            let value_high = ((sysreg_value >> 64) & 0xFFFFFFFFFFFFFFFF) as u64;
            (RSI_OK, value_low, value_high)
        }
    }
}
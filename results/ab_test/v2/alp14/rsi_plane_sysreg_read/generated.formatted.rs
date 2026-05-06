pub open spec fn rsi_plane_sysreg_read_spec(
    result: RsiCommandReturnCode,
    value_low: u64,
    value_high: u64,
    old_s: S,
    new_s: S,
    plane_idx: u64,
    addr: RsiSysregAddress
) -> bool {
    let realm = current_realm(old_s);
    let rec = current_rec(old_s);
    
    // Failure condition: idx_bound
    (plane_idx > realm.num_aux_planes ==> result == RSI_ERROR_INPUT)
    
    // Failure condition: sysreg_valid
    && (!plane_sysreg_valid(rec, addr, RMM_READ) ==> result == RSI_ERROR_INPUT)
    
    // Success condition: value_64
    && (addr.d128 == RSI_FALSE && plane_idx <= realm.num_aux_planes && plane_sysreg_valid(rec, addr, RMM_READ)
        ==> {
            let expected_value = plane_sysreg_value(rec, plane_idx, addr);
            (zeros(64) :: value_low) == expected_value && value_high == 0
        })
    
    // Success condition: value_128
    && (addr.d128 == RSI_TRUE && plane_idx <= realm.num_aux_planes && plane_sysreg_valid(rec, addr, RMM_READ)
        ==> {
            let expected_value = plane_sysreg_value(rec, plane_idx, addr);
            (value_high :: value_low) == expected_value
        })
    
    // No state changes
    && new_s == old_s
}
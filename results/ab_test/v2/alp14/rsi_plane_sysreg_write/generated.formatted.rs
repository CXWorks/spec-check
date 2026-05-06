pub open spec fn rsi_plane_sysreg_write_spec(
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
) -> bool {
    let realm = current_realm(old_s);
    let rec = current_rec(old_s);

    // Failure condition: idx_bound
    (plane_idx > realm.num_aux_planes ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: sysreg_valid
     && (!plane_sysreg_valid(rec, addr, RMM_WRITE) ==> result
        == RSI_ERROR_INPUT)
    // Success conditions
     && ((plane_idx <= realm.num_aux_planes && plane_sysreg_valid(rec, addr, RMM_WRITE)) ==> {
        // Success: value_low written
        let sysreg_value = plane_sysreg_value(new_s, rec, plane_idx, addr);
        let value_low_match = (sysreg_value & 0xFFFFFFFFFFFFFFFF) == value_low;

        // Success: value_high written (if d128)
        let value_high_match = if addr.d128 == RSI_TRUE {
            ((sysreg_value >> 64) & 0xFFFFFFFFFFFFFFFF) == value_high
        } else {
            true
        };

        // Footprint: only rec.sysregs changed
        value_low_match && value_high_match && rec_sysregs_changed_only(old_s, new_s, rec)
    })
}
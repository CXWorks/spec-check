pub open spec fn rsi_mem_set_perm_value_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, realm: RmmRealm, plane_index: u64, perm_index: u64, value: u64) -> bool {
    let plane_bound = plane_index == 0 || plane_index > realm.num_aux_planes;
    let perm_bound = perm_index >= RMM_NUM_PERM_OVERLAY_INDICES;
    let locked = old_s.overlay_locked[perm_index] == MEM_PERM_LOCKED;
    let supported = MemPermLabelSupported(value);
    
    (!plane_bound && !perm_bound && !locked && supported ==>
        (result.is_Ok() && new_s.overlay_perms[plane_index].values[perm_index] == value))
    && (plane_bound ==> result == RSI_ERROR_INPUT)
    && (perm_bound && !plane_bound ==> result == RSI_ERROR_INPUT)
    && (locked && !plane_bound && !perm_bound ==> result == RSI_ERROR_INPUT)
    && (!supported && !plane_bound && !perm_bound && !locked ==> result == RSI_ERROR_INPUT)
}
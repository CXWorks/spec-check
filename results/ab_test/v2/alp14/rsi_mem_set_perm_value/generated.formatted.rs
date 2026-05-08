pub open spec fn rsi_mem_set_perm_value_spec(
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
    plane_index: u64,
    perm_index: u64,
    value: u64,
) -> bool {
    let realm = old_s.current_realm();

    // Failure condition: plane_bound
    (plane_index == 0 || plane_index > realm.num_aux_planes ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: perm_bound
    (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: locked
    (realm.overlay_locked[perm_index as int] == MEM_PERM_LOCKED ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: supported
    (!MemPermLabelSupported(old_s, value) ==> result == RSI_ERROR_INPUT)
        &&
    // Success condition: label
    ((plane_index > 0 && plane_index <= realm.num_aux_planes && perm_index
        < RMM_NUM_PERM_OVERLAY_INDICES && realm.overlay_locked[perm_index as int] != MEM_PERM_LOCKED
        && MemPermLabelSupported(old_s, value)) ==> (result == RSI_OK
        && new_s.current_realm().overlay_perms[plane_index as int].values[perm_index as int]
        == value))
}
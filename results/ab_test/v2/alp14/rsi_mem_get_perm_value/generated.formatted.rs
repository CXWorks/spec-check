pub open spec fn rsi_mem_get_perm_value_spec(
    result: RsiCommandReturnCode,
    value: u64,
    old_s: S,
    new_s: S,
    plane_index: u64,
    perm_index: u64,
) -> bool {
    let realm = CurrentRealm(old_s);

    // Failure condition: plane_bound
    (plane_index > realm.num_aux_planes as u64 ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: perm_bound
     && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES as u64 ==> result
        == RSI_ERROR_INPUT)
    // Success condition: label
     && (plane_index <= realm.num_aux_planes as u64 && perm_index
        < RMM_NUM_PERM_OVERLAY_INDICES as u64 ==> (result == RSI_SUCCESS && value
        == realm.overlay_perms[plane_index as int].values[perm_index as int]))
    // No state changes
     && new_s == old_s
}
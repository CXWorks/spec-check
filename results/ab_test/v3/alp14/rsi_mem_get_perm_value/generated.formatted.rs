pub open spec fn RSI_MEM_GET_PERM_VALUE_spec(
    old_s: S,
    new_s: S,
    plane_index: u64,
    perm_index: u64,
    result: RsiCommandReturnCode,
    value: u64,
) -> bool {
    let realm = old_s.current_realm();

    // Failure condition: plane_bound
    (plane_index > realm.num_aux_planes ==> result == RSI_ERROR_INPUT)
        &&
    // Failure condition: perm_bound
    (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
        &&
    // Success condition: when no failures occur
    ((plane_index <= realm.num_aux_planes && perm_index < RMM_NUM_PERM_OVERLAY_INDICES) ==> (result
        == RSI_SUCCESS && value
        == realm.overlay_perms[plane_index as int].values[perm_index as int]))
        &&
    // State unchanged
    (new_s == old_s)
}
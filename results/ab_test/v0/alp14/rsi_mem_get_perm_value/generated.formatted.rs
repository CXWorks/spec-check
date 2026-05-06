pub open spec fn RSI_MEM_GET_PERM_VALUE_spec(
    s: S,
    realm: RmmRealm,
    plane_index: u64,
    perm_index: u64,
    result: RsiCommandReturnCode,
    value: u64,
) -> bool {
    if plane_index > realm.num_aux_planes as u64 {
        result == RSI_ERROR_INPUT
    } else if perm_index >= RMM_NUM_PERM_OVERLAY_INDICES as u64 {
        result == RSI_ERROR_INPUT
    } else {
        result == RSI_SUCCESS && value
            == realm.overlay_perms[plane_index as int].values[perm_index as int]
    }
}
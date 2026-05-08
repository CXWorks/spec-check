pub open spec fn RSI_MEM_GET_PERM_VALUE_spec(
    old_s: S,
    new_s: S,
    realm: RmmRealm,
    plane_index: u64,
    perm_index: u64,
    result: RsiCommandReturnCode,
    value: u64,
) -> bool {
    (plane_index > realm.num_aux_planes ==> result == RSI_ERROR_INPUT) && (perm_index
        >= RMM_NUM_PERM_OVERLAY_INDICES as int ==> result == RSI_ERROR_INPUT) && (plane_index
        <= realm.num_aux_planes && perm_index < RMM_NUM_PERM_OVERLAY_INDICES as int ==> result
        == RSI_SUCCESS && value
        == realm.overlay_perms[plane_index as int].values[perm_index as int]) && old_s == new_s
}
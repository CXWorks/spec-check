pub open spec fn rsi_mem_get_perm_value_spec(
    result: RsiCommandReturnCode,
    value: u64,
    old_s: S,
    new_s: S,
    plane_index: u64,
    perm_index: u64,
) -> bool {
    (plane_index > old_s.realm.num_aux_planes ==> result == RSI_ERROR_INPUT) && (perm_index
        >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT) && ((plane_index
        <= old_s.realm.num_aux_planes && perm_index < RMM_NUM_PERM_OVERLAY_INDICES) ==> (result
        == RSI_SUCCESS && value
        == old_s.realm.overlay_perms[plane_index as int].values[perm_index as int] && new_s
        == old_s))
}
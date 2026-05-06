pub open spec fn rsi_mem_get_perm_value_spec(
    plane_index: UInt64,
    perm_index: UInt64,
    result: RsiCommandReturnCode,
    value: Bits64,
    old_s: S,
    new_s: S,
) -> bool {
    (plane_index > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT) && (perm_index
        >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT) && (result == RSI_SUCCESS
        ==> value == CurrentRealm(
        new_s,
    ).overlay_perms[plane_index as int].values[perm_index as int]) && ((!(plane_index
        > CurrentRealm(old_s).num_aux_planes) && !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES))
        ==> result == RSI_SUCCESS)
}
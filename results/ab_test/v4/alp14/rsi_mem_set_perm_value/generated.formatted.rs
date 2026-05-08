pub open spec fn RSI_MEM_SET_PERM_VALUE_spec(
    old_s: S,
    new_s: S,
    plane_index: u64,
    perm_index: u64,
    value: u64,
    realm: RmmRealm,
    result: RsiCommandReturnCode,
) -> bool {
    ((plane_index == 0 || plane_index > realm.num_aux_planes) ==> result
        == RsiCommandReturnCode::RSI_ERROR_INPUT) && ((perm_index >= RMM_NUM_PERM_OVERLAY_INDICES)
        ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT) && ((
    realm.overlay_locked[perm_index as int] == MEM_PERM_LOCKED) ==> result
        == RsiCommandReturnCode::RSI_ERROR_INPUT) && ((!MemPermLabelSupported(old_s, value))
        ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT) && ((!(plane_index == 0 || plane_index
        > realm.num_aux_planes) && !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) && !(
    realm.overlay_locked[perm_index as int] == MEM_PERM_LOCKED) && MemPermLabelSupported(
        old_s,
        value,
    )) ==> (result == RsiCommandReturnCode::RSI_SUCCESS
        && new_s.realms[realm].overlay_perms[plane_index as int].values[perm_index as int]
        == value))
}
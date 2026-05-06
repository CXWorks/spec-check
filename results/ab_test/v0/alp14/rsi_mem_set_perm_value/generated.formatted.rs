pub open spec fn RSI_MEM_SET_PERM_VALUE_spec(
    s: S,
    realm: RmmRealm,
    plane_index: u64,
    perm_index: u64,
    value: u64,
    result: RsiCommandReturnCode,
) -> bool {
    let plane_index_valid = plane_index != 0 && plane_index <= realm.num_aux_planes;
    let perm_index_valid = perm_index < RMM_NUM_PERM_OVERLAY_INDICES;
    let not_locked = realm.overlay_locked[perm_index as int] != MEM_PERM_LOCKED;
    let supported = MemPermLabelSupported(value);

    (plane_index_valid && perm_index_valid && not_locked && supported) ==> (
    realm.overlay_perms[plane_index as int].values[perm_index as int] == value)
}
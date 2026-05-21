pub open spec fn rsi_mem_set_perm_value_spec(result: RsiCommandReturnCode, plane_index: u64, perm_index: u64, value: u64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    (
        (plane_index == 0 || plane_index > realm.num_aux_planes) ==> result == RSI_ERROR_INPUT
    ) && (
        perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT
    ) && (
        realm.overlay_locked[perm_index] == MEM_PERM_LOCKED ==> result == RSI_ERROR_INPUT
    ) && (
        !MemPermLabelSupported(value) ==> result == RSI_ERROR_INPUT
    ) && (
        (plane_index > 0 && plane_index <= realm.num_aux_planes && perm_index < RMM_NUM_PERM_OVERLAY_INDICES && realm.overlay_locked[perm_index] != MEM_PERM_LOCKED && MemPermLabelSupported(value))
        ==> (result == RSI_SUCCESS && new_s.realm.overlay_perms[plane_index].values[perm_index] == value)
    )
}
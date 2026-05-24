pub open spec fn rsi_mem_get_perm_value_spec(result: RsiCommandReturnCode, value: u64, old_s: S, new_s: S, plane_index: u64, perm_index: u64) -> bool {
    let realm = old_s.current_realm();
    (plane_index > realm.num_aux_planes ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && ((plane_index <= realm.num_aux_planes && perm_index < RMM_NUM_PERM_OVERLAY_INDICES) ==> (result.is_Ok() && value == realm.overlay_perms[plane_index as int].values[perm_index as int]))
    && (new_s == old_s)
}
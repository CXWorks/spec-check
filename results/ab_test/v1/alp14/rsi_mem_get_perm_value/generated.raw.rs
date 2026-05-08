```verus
pub open spec fn RSI_MEM_GET_PERM_VALUE_spec(s: S, plane_index: u64, perm_index: u64) -> (result: RsiCommandReturnCode, value: u64) {
    let realm = CurrentRealm(s);
    if plane_index > realm.num_aux_planes {
        (RSI_ERROR_INPUT, 0u64)
    } else if perm_index >= RMM_NUM_PERM_OVERLAY_INDICES {
        (RSI_ERROR_INPUT, 0u64)
    } else {
        (RSI_SUCCESS, realm.overlay_perms[plane_index as int].values[perm_index as int])
    }
}
```
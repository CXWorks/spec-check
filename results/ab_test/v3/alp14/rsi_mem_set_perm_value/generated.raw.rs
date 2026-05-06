```verus
pub open spec fn RSI_MEM_SET_PERM_VALUE_spec(
    old_s: S,
    new_s: S,
    plane_index: u64,
    perm_index: u64,
    value: u64,
    result: RsiCommandReturnCode,
) -> bool {
    let realm = CurrentRealm(old_s);
    
    // Failure condition: plane_bound
    (plane_index == 0 || plane_index > realm.num_aux_planes ==> result == RSI_ERROR_INPUT) &&
    
    // Failure condition: perm_bound
    (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT) &&
    
    // Failure condition: locked
    (realm.overlay_locked[perm_index as int] == MEM_PERM_LOCKED ==> result == RSI_ERROR_INPUT) &&
    
    // Failure condition: supported
    (!MemPermLabelSupported(old_s, value) ==> result == RSI_ERROR_INPUT) &&
    
    // Success condition: label
    (result == RSI_OK ==>
        new_s.realm.overlay_perms[plane_index as int].values[perm_index as int] == value &&
        // State unchanged except for the overlay permission value
        forall|i: int, j: int| (i != plane_index as int || j != perm_index as int) ==>
            old_s.realm.overlay_perms[i].values[j] == new_s.realm.overlay_perms[i].values[j])
}
```
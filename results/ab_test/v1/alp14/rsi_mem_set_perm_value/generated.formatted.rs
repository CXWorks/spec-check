```verus
pub open spec fn RSI_MEM_SET_PERM_VALUE_spec(
    s: S,
    plane_index: u64,
    perm_index: u64,
    value: u64,
) -> (result: RsiCommandReturnCode, s_prime: S)
{
    let realm = current_realm(s);
    
    if plane_index == 0 || plane_index > realm.num_aux_planes {
        (RSI_ERROR_INPUT, s)
    } else if perm_index >= RMM_NUM_PERM_OVERLAY_INDICES {
        (RSI_ERROR_INPUT, s)
    } else if realm.overlay_locked[perm_index] == MEM_PERM_LOCKED {
        (RSI_ERROR_INPUT, s)
    } else if !mem_perm_label_supported(s, value) {
        (RSI_ERROR_INPUT, s)
    } else {
        let s_prime = s.with_overlay_perm_value(plane_index, perm_index, value);
        (RSI_SUCCESS, s_prime)
    }
}
```

This specification captures:

1. **Input parameters**: `plane_index`, `perm_index`, and `value` from X1, X2, and X3 registers
2. **Context**: Uses the current realm from the execution state `s`
3. **Failure conditions** (checked in order):
   - `plane_bound`: Plane index out of valid range (0 or > num_aux_planes)
   - `perm_bound`: Permission index >= RMM_NUM_PERM_OVERLAY_INDICES
   - `locked`: The overlay permission is already locked
   - `supported`: The value is not a supported memory permission label
4. **Success condition**: Updates the overlay permission value at the specified plane and permission indices
5. **Return values**: Command return code and updated state

The function returns both the result status and the updated state, allowing it to express both the output register value and the side effects of the operation.
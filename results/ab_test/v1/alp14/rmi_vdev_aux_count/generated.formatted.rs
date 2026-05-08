```rust
pub open spec fn RMI_VDEV_AUX_COUNT_spec(s: S, pdev_flags: u64, vdev_flags: u64) -> (result: RmiCommandReturnCode, aux_count: u64) {
    if !ImplFeatures(s).feat_da {
        (RMI_ERROR_NOT_SUPPORTED, 0)
    } else {
        let pdev_flags_decoded = RmiPdevFlagsDecode(s, pdev_flags);
        let vdev_flags_decoded = RmiVdevFlagsDecode(s, vdev_flags);
        let aux_count_value = VdevAuxCount(s, pdev_flags_decoded, vdev_flags_decoded);
        (RMI_SUCCESS, aux_count_value as u64)
    }
}
```

This specification function:
- Takes the system state `s`, `pdev_flags`, and `vdev_flags` as inputs
- Returns a tuple of `(result, aux_count)` corresponding to X0 and X1 output registers
- Checks the failure condition: if `feat_da` is not supported, returns `RMI_ERROR_NOT_SUPPORTED`
- Otherwise, decodes both flags using the provided decode functions
- Computes the auxiliary count using `VdevAuxCount`
- Returns success with the computed auxiliary count
```verus
pub open spec fn rmi_vdev_aux_count_spec(
    result: RmiCommandReturnCode,
    aux_count: u64,
    old_s: S,
    new_s: S,
    pdev_flags: u64,
    vdev_flags: u64
) -> bool {
    // Failure condition: da_supp
    ((!ImplFeatures(old_s).feat_da_eq_true()) ==> 
        ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    &&
    // Success condition: aux_count
    ((ImplFeatures(old_s).feat_da_eq_true()) ==> 
        aux_count == VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags), 
                                  RmiVdevFlagsDecode(old_s, vdev_flags)) as u64)
    &&
    // No state change
    (new_s == old_s)
}
```
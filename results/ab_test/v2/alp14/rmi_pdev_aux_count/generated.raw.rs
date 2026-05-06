```verus
pub open spec fn rmi_pdev_aux_count_spec(result: RmiCommandReturnCode, aux_count: u64, old_s: S, new_s: S, value: u64) -> bool {
    let flags = RmiPdevFlagsDecode(old_s, value);
    
    // Failure condition: da_supp
    (ImplFeatures().feat_da != FEATURE_TRUE ==> result == RMI_ERROR_NOT_SUPPORTED)
    
    // Failure condition: flags_supp
    && (!RmiPdevFlagsSupported(old_s, flags) ==> result == RMI_ERROR_INPUT)
    
    // Success condition: aux_count
    && ((ImplFeatures().feat_da == FEATURE_TRUE && RmiPdevFlagsSupported(old_s, flags))
        ==> aux_count == VdevAuxCount(old_s, flags, RmiVdevFlagsDecode(old_s, 0)))
    
    // State unchanged
    && (new_s == old_s)
}
```
pub open spec fn RMI_PDEV_AUX_COUNT_spec(
    old_s: S,
    new_s: S,
    value: u64,
    result: RmiCommandReturnCode,
    aux_count: u64,
) -> bool {
    let flags = RmiPdevFlagsDecode(old_s, value);

    // Failure condition: da_supp
    (old_s.ImplFeatures().feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // Failure condition: flags_supp
    (!RmiPdevFlagsSupported(old_s, flags) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Success condition: aux_count
    ((old_s.ImplFeatures().feat_da == FEATURE_TRUE && RmiPdevFlagsSupported(old_s, flags)) ==> (
    aux_count == VdevAuxCount(
        old_s,
        flags,
        RmiVdevFlags {   /* empty/default */  },
    ))) &&
    // No state changes
    new_s == old_s
}
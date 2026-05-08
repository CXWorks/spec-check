```verus
pub open spec fn RMI_PDEV_AUX_COUNT_spec(s: S, value: u64, result: Result<(), RmiStatusCode>, aux_count: u64) -> bool {
    let flags = RmiPdevFlagsDecode(s, value);
    (
        // da_supp failure condition
        (s.impl_features().feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    ) && (
        // flags_supp failure condition
        (!RmiPdevFlagsSupported(s, flags) ==> ResultEqual(result, RMI_ERROR_INPUT))
    ) && (
        // aux_count success condition
        (result.is_Ok() ==> aux_count == VdevAuxCount(s, flags, RmiVdevFlags::default()))
    )
}
```
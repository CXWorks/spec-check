pub open spec fn RMI_PDEV_AUX_COUNT_spec(s: S, value: u64, result: Result<(), RmiStatusCode>, aux_count: u64) -> bool {
    let flags = RmiPdevFlagsDecode(s, value);
    ((!ImplFeatures(s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    ((!RmiPdevFlagsSupported(s, flags)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((ImplFeatures(s).feat_da && RmiPdevFlagsSupported(s, flags)) ==> (result.is_Ok() && aux_count as int == VdevAuxCount(s, flags, RmiVdevFlagsDecode(s, 0))))
}
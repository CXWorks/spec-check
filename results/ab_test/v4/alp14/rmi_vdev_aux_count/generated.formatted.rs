pub open spec fn RMI_VDEV_AUX_COUNT_spec(
    s: S,
    pdev_flags: Bits64,
    vdev_flags: Bits64,
    result: RmiStatusCode,
    aux_count: UInt32,
) -> bool {
    ((!ImplFeatures(s).feat_da) ==> ResultEqual(Err(result), RMI_ERROR_NOT_SUPPORTED)) && (
    ImplFeatures(s).feat_da ==> (result == RMI_SUCCESS && aux_count == VdevAuxCount(
        s,
        RmiPdevFlagsDecode(s, pdev_flags as int),
        RmiVdevFlagsDecode(s, vdev_flags as int),
    )))
}
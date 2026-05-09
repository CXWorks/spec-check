pub open spec fn rmi_pdev_aux_count_spec(
    result: RmiCommandReturnCode,
    aux_count: u64,
    value: u64,
    old_s: S,
    new_s: S,
) -> bool {
    let flags = RmiPdevFlagsDecode(old_s, value);
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!RmiPdevFlagsSupported(old_s, flags) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    ImplFeatures(old_s).feat_da == FEATURE_TRUE && RmiPdevFlagsSupported(old_s, flags)) ==> (result
        == RMI_OK && aux_count == ToBits64(
        VdevAuxCount(old_s, flags, RmiVdevFlags { reserved: 0 }),
    ))) && old_s == new_s
}
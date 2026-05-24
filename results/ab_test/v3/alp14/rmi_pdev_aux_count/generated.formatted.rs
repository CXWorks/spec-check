pub open spec fn rmi_pdev_aux_count_spec(result: RmiCommandReturnCode, aux_count: u64, old_s: S, new_s: S) -> bool {
    ((!ImplFeatures(old_s).feat_da.is_true()) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((!RmiPdevFlagsSupported(old_s, RmiPdevFlagsDecode(old_s, result))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((result.is_Ok() && ImplFeatures(old_s).feat_da.is_true() && RmiPdevFlagsSupported(old_s, RmiPdevFlagsDecode(old_s, result))) ==> aux_count == VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, result), RmiVdevFlags::default()))
}
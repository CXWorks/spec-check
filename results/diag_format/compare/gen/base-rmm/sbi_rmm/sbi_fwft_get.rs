pub open spec fn sbi_fwft_get_spec(result: SbiCommandReturnCode, value: u32, feature: u32, old_s: S, new_s: S) -> bool {
    (!IsReservedFeature(feature) && IsValidFeature(feature) && !PlatformSupportsFeature(feature) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED) && value == 0)
    && (IsReservedFeature(feature) || (IsPlatformSpecificFeature(feature) && !IsImplementedFeature(feature)) ==> ResultEqual(result, SBI_ERR_DENIED) && value == 0)
    && (FeatureGetFailed(feature) ==> ResultEqual(result, SBI_ERR_FAILED) && value == 0)
    && (ResultEqual(result, SBI_SUCCESS) ==> value == FeatureConfigValue(feature))
}
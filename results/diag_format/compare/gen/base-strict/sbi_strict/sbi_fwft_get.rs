pub open spec fn sbi_fwft_get_spec(result: SbiErrorCode, value: UInt, old_s: S, new_s: S) -> bool {
    (!IsReservedFeature(old_s, feature) && IsValidFeature(old_s, feature) && !PlatformSupportsFeature(old_s, feature) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED) && value == 0)
    && (IsReservedFeature(old_s, feature) || (IsPlatformSpecificFeature(old_s, feature) && !IsImplementedFeature(old_s, feature)) ==> ResultEqual(result, SBI_ERR_DENIED) && value == 0)
    && (FeatureGetFailed(old_s, feature) ==> ResultEqual(result, SBI_ERR_FAILED) && value == 0)
    && (ResultEqual(result, SBI_SUCCESS) ==> value == FeatureConfigValue(old_s, feature))
    && (old_s == new_s)
}
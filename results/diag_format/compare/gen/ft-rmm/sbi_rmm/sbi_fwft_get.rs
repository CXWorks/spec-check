pub open spec fn sbi_fwft_get_spec(feature: uint32_t, result: SbiCommandReturnCode, value: uint64_t, old_s: S, new_s: S) -> bool {
  (!IsReservedFeature(old_s, feature) && IsValidFeature(old_s, feature) && !PlatformSupportsFeature(old_s, feature) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (result == SBI_ERR_NOT_SUPPORTED ==> value == 0)
  && (IsReservedFeature(old_s, feature) || (IsPlatformSpecificFeature(old_s, feature) && !IsImplementedFeature(old_s, feature)) ==> ResultEqual(result, SBI_ERR_DENIED))
  && (result == SBI_ERR_DENIED ==> value == 0)
  && (FeatureGetFailed(old_s, feature) ==> ResultEqual(result, SBI_ERR_FAILED))
  && (result == SBI_ERR_FAILED ==> value == 0)
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> value == FeatureConfigValue(old_s, feature))
  && ((!(IsReservedFeature(old_s, feature) && IsValidFeature(old_s, feature) && !PlatformSupportsFeature(old_s, feature)) &&
       !(IsReservedFeature(old_s, feature) || (IsPlatformSpecificFeature(old_s, feature) && !IsImplementedFeature(old_s, feature))))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> value == 0)
}
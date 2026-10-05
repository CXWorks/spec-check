pub open spec fn sbi_fwft_get_spec(feature: UInt32, error: i64, value: UInt64, old_s: S, new_s: S) -> bool {
    (error == SBI_SUCCESS || error == SBI_ERR_NOT_SUPPORTED || error == SBI_ERR_DENIED || error == SBI_ERR_FAILED)
    && ((FwftFeatureIsReserved(feature) || (FwftFeatureIsPlatformSpecific(feature) && !FwftFeatureIsImplemented(old_s, feature))) ==> error == SBI_ERR_DENIED)
    && ((!FwftFeatureIsReserved(feature) && FwftFeatureIsValid(feature) && !FwftFeatureIsSupported(old_s, feature)) ==> error == SBI_ERR_NOT_SUPPORTED)
    && (error != SBI_SUCCESS ==> value == 0)
    && (error == SBI_SUCCESS ==> (!FwftFeatureIsReserved(feature) && FwftFeatureIsValid(feature) && FwftFeatureIsSupported(old_s, feature) && value == FwftFeatureValue(old_s, feature)))
    && (new_s == old_s)
}

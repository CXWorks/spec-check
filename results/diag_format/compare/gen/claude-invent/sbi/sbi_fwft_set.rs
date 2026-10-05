pub open spec fn sbi_fwft_set_spec(feature: UInt32, value: UInt64, flags: UInt64, ret: SbiRet, old_s: S, new_s: S) -> bool {
    (((flags >> 1u64) != 0u64) ==> ret.error != SBI_SUCCESS)
    && ((FwftFeatureLocked(old_s, feature) && FwftFeatureValue(old_s, feature) != value) ==> ret.error != SBI_SUCCESS)
    && (((flags >> 1u64) == 0u64
            && FwftFeatureSupported(old_s, feature)
            && FwftFeatureValue(old_s, feature) == value)
        ==> ret.error == SBI_SUCCESS)
    && (ret.error == SBI_SUCCESS ==> (
            (flags >> 1u64) == 0u64
            && FwftFeatureSupported(old_s, feature)
            && FwftFeatureValueSupported(feature, value)
            && FwftFeatureValue(new_s, feature) == value
            && ((flags & 1u64) == 1u64 ==> FwftFeatureLocked(new_s, feature))
            && (FwftFeatureLocked(old_s, feature) ==> FwftFeatureLocked(new_s, feature))
        ))
    && (ret.error != SBI_SUCCESS ==> (
            FwftFeatureValue(new_s, feature) == FwftFeatureValue(old_s, feature)
            && FwftFeatureLocked(new_s, feature) == FwftFeatureLocked(old_s, feature)
        ))
}

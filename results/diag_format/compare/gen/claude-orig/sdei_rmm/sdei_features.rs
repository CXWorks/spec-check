pub open spec fn sdei_features_spec(feature: UInt32, result: Int64, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidSdeiFeature(feature) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((IsSdeiSupported() && IsValidSdeiFeature(feature) && feature == BIND_SLOTS) ==> (
        (((result as u64) >> 32u64) == 0u64)
        && ((((result as u64) >> 16u64) & 0xFFFFu64) == (SharedEventSlotCount() as u64))
        && (((result as u64) & 0xFFFFu64) == (PrivateEventSlotCount() as u64))
    ))
    && ((IsSdeiSupported() && IsValidSdeiFeature(feature) && feature == RELATIVE_MODE && IsRelativeModeSupported()) ==> (result == 1))
    && ((IsSdeiSupported() && IsValidSdeiFeature(feature) && feature == RELATIVE_MODE && !IsRelativeModeSupported()) ==> (result == 0))
    && (new_s == old_s)
}

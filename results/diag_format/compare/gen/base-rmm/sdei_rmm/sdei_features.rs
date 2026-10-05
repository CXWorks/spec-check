pub open spec fn sdei_features_spec(result: Int64, feature: UInt32, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidSdeiFeature(feature) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (feature == BIND_SLOTS ==> (result[63:32] == 0 && result[31:16] == SharedEventSlotCount() && result[15:0] == PrivateEventSlotCount()))
    && (feature == RELATIVE_MODE && IsRelativeModeSupported() ==> result == 1)
    && (feature == RELATIVE_MODE && !IsRelativeModeSupported() ==> result == 0)
}
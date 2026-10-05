pub open spec fn sdei_features_spec(result: Int64, feature: UInt32, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsDefinedSdeiFeature(feature) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (feature == BIND_SLOTS ==> Bits(result, 63, 32) == 0)
    && (feature == BIND_SLOTS ==> Bits(result, 31, 16) == SharedEventSlotCount())
    && (feature == BIND_SLOTS ==> Bits(result, 15, 0) == PrivateEventSlotCount())
    && (feature == RELATIVE_MODE && RelativeModeSupported() ==> result == 1)
    && (feature == RELATIVE_MODE && !RelativeModeSupported() ==> result == 0)
    && (new_s == old_s)
}
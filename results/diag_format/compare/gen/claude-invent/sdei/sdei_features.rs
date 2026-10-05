pub open spec fn sdei_features_spec(feature: u32, result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiIsSupported(old_s) ==> result == NOT_SUPPORTED)
    && ((SdeiIsSupported(old_s) && feature != 0 && feature != 1) ==> result == INVALID_PARAMETERS)
    && ((SdeiIsSupported(old_s) && feature == 0) ==> (
        (result as int) >= 0
        && (result as int) < 0x1_0000_0000
        && ((result as int) / 0x1_0000) % 0x1_0000 == SdeiSharedEventSlotCount(old_s) as int
        && (result as int) % 0x1_0000 == SdeiPrivateEventSlotCount(old_s) as int
    ))
    && ((SdeiIsSupported(old_s) && feature == 1) ==> (
        (result == 0 || result == 1)
        && (result == 1 <==> SdeiRelativeModeSupported(old_s))
    ))
    && new_s == old_s
}

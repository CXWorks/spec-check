pub open spec fn sdei_features_spec(result: int64, feature: u32, old_s: S, new_s: S) -> bool {
    (feature == 0u32 ==> (result as int >= 0 && (result as int) < (1u64 << 64)))
    && (feature != 0u32 ==> result == NOT_SUPPORTED)
    && (old_s == new_s)
}
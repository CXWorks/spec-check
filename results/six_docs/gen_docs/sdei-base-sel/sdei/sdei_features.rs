pub open spec fn sdei_features_spec(result: int64, feature: uint32, old_s: S, new_s: S) -> bool {
    (feature == 0 ==> (result as int64 >= 0 && (result as int64) < (1u64 << 32) && (result >> 32) == 0))
    && (feature == 1 ==> (result == 0 || result == 1))
    && (feature != 0 && feature != 1 ==> result == SDEI_INVALID_PARAMETERS)
    && (old_s == new_s)
}
pub open spec fn sdei_features_spec(result: int64, feature: UInt32, old_s: S, new_s: S) -> bool {
    (feature == 0 ==> (result as int >= 0))
    && (feature == 1 ==> (result == 0 || result == 1))
    && (feature != 0 && feature != 1 ==> result == SDEI_INVALID_PARAMETERS)
    && (result == SDEI_INVALID_PARAMETERS ==> feature != 0 && feature != 1)
    && (result == SDEI_NOT_SUPPORTED ==> true)
    && (result == SDEI_SUCCESS ==> (feature == 0 || feature == 1))
}
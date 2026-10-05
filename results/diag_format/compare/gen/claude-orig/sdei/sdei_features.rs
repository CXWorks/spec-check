pub open spec fn sdei_features_spec(feature: u32, result: i64, old_s: S, new_s: S) -> bool {
    ((feature != 0 && feature != 1) ==> result < 0)
    && ((feature == 0 && result >= 0) ==> (result as int) < 0x1_0000_0000)
    && ((feature == 1 && result >= 0) ==> (result == 0 || result == 1))
    && (new_s == old_s)
}

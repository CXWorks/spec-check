pub open spec fn sdei_version_spec(result: i64, old_s: S, new_s: S) -> bool {
    (!SdeiSupported(old_s) ==> result == NOT_SUPPORTED)
    && (SdeiSupported(old_s) ==> (
        result >= 0
        && (((result as u64) >> 48u64) & 0x7FFFu64) == 1u64
        && (((result as u64) >> 32u64) & 0xFFFFu64) == 1u64
    ))
    && new_s == old_s
}

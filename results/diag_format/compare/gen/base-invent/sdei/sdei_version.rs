pub open spec fn sdei_version_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result < 0 ==> ResultEqual(result, SDEI_ERROR_NOT_SUPPORTED))
    && (result >= 0 ==> (result as int64 < (1u64 << 63)) && (result as int64 >= (1u64 << 48)) && (result as int64 < (1u64 << 63)))
}
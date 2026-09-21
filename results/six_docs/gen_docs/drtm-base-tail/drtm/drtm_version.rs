pub open spec fn drtm_version_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == DRTM_NOT_SUPPORTED ==> true)
    && (result != DRTM_NOT_SUPPORTED ==> (result as int >= 0 && result as int < (1u64 << 31) && (result as int) >> 16 == 1 && (result as int) & 0xFFFF == 4))
}
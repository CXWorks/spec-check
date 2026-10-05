pub open spec fn drtm_version_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (result != NOT_SUPPORTED ==> (
        ((result >> 31u32) & 1u32) == 0u32
        && ((result >> 16u32) & 0x7FFFu32) == 1u32
        && (result & 0xFFFFu32) == 4u32
    ))
    && (new_s == old_s)
}

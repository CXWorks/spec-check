pub open spec fn system_reset2_spec(reset_type: UInt32, cookie: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsSystemReset2Implemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((IsSystemReset2Implemented(old_s)
         && (reset_type & 0x8000_0000u32) == 0u32
         && (reset_type & 0x7FFF_FFFFu32) != 0u32)
        ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((IsSystemReset2Implemented(old_s)
         && (reset_type & 0x8000_0000u32) != 0u32)
        ==> result != NOT_SUPPORTED)
}

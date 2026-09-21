pub open spec fn 3.12.4.8_telemetry_de_configure_spec(
    result: int32,
    identifier: uint32,
    flags: uint32,
    old_s: S,
    new_s: S,
) -> bool {
    // Failure conditions
    // Reserved bits [31:4] must be zero
    (flags & 0xFFFF_FFFF_0000_0000 != 0 ==> result == INVALID_PARAMETERS)
    // If Disable All (Bit[2] == 1), then DE Mode (Bits[1:0]) must be 0
    ((flags & 0x08) != 0 && (flags & 0x03) != 0 ==> result == INVALID_PARAMETERS)
    // Success conditions
    // If Disable All is set, identifier is ignored, and all DEs/groups are disabled
    ((flags & 0x08) != 0 ==> (result == SUCCESS && new_s.de_count == old_s.de_count))
    // If Disable All is not set, the specific DE or event group is configured
    ((flags & 0x08) == 0 ==> (result == SUCCESS || result == IN_USE || result == OUT_OF_RANGE))
    // If successful configuration (not Disable All), SHMTI info is returned if applicable
    ((result == SUCCESS && (flags & 0x08) == 0) ==> (
        (new_s.shmti_id == 0xFFFFFFFF || new_s.shmti_id == old_s.shmti_id)
        && (new_s.shmti_de_offset == 0 || new_s.shmti_de_offset == old_s.shmti_de_offset)
        && (new_s.blk_ts_offset == 0 || new_s.blk_ts_offset == old_s.blk_ts_offset)
    ))
}
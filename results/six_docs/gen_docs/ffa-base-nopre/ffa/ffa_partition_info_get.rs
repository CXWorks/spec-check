pub open spec fn ffa_partition_info_get_spec(
    result: FfaReturnCode,
    function_id: UInt32,
    uuid: [UInt8; 16],
    flags: UInt32,
    rx_buffer: *mut UInt8,
    rx_buffer_size: UInt32,
    old_s: S,
    new_s: S,
) -> bool {
    (function_id == 0x84000068)
    && (uuid.iter().all(|&b| b == 0))
    && (flags & 0x1 == 0)
    && (rx_buffer_size >= 1)
    && (result == FFA_SUCCESS)
    && (new_s == old_s)
}
pub open spec fn ffa_ns_res_info_get_spec(
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
    function_id: u32,
    target_id: u64,
    flags: u64,
    rx_buffer: *const u8,
    rx_buffer_len: u64,
) -> bool {
    // Failure conditions
    (function_id != 0xC400008F ==> result == RSI_ERROR_INPUT)
    && (target_id & 0xFFFF_0000_0000_0000 != 0 ==> result == RSI_ERROR_INPUT)
    && ((flags & 0x3E0) != 0 ==> result == RSI_ERROR_INPUT)
    && ((flags & 0x1) != 0 && (target_id & 0xFFFF) == 0 ==> result == RSI_ERROR_INPUT)
    && ((flags & 0x1) == 0 && (target_id & 0xFFFF) != 0 ==> result == RSI_ERROR_INPUT)
    && (rx_buffer == null_ptr ==> result == RSI_ERROR_INPUT)
    && (rx_buffer_len == 0 && (flags & 0x1) == 0 ==> result == RSI_ERROR_INPUT)
    && (rx_buffer_len == 0 && (flags & 0x1) != 0 ==> result == RSI_ERROR_INPUT)
    // Success conditions
    && (result == RSI_SUCCESS ==>
        (
            // If Target S-Endpoint ID valid flag is 0, entire NS PAS is inaccessible
            ((flags & 0x1) == 0 && rx_buffer_len == 0)
            ||
            // If Target S-Endpoint ID valid flag is 1, and entire NS PAS is inaccessible from that endpoint
            ((flags & 0x1) != 0 && rx_buffer_len == 0)
            ||
            // If data is returned, it must be properly formatted
            (rx_buffer_len > 0 &&
                // Resource information descriptor header must be present
                (rx_buffer as *const u8).offset(0).as_ref().map_or(false, |h| {
                    // Check header structure (simplified validation based on spec)
                    // Address map descriptor size and count must be valid
                    let size = (h as *const u8).offset(0).as_ref().map_or(0, |sz| *sz as u32);
                    let count = (h as *const u8).offset(4).as_ref().map_or(0, |cnt| *cnt as u32);
                    let offset = (h as *const u8).offset(8).as_ref().map_or(0, |off| *off as u32);
                    // Basic sanity checks
                    size > 0 && count > 0 && offset > 0 && offset % 16 == 0
                }))
        )
    )
}
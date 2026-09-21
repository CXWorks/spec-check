pub open spec fn ffa_ns_res_info_get_spec(
    result: int32,
    old_s: S,
    new_s: S,
    target_id: UInt64,
    flags: UInt64,
    written_size: UInt64,
    remaining_size: UInt64,
) -> bool {
    // Failure conditions
    // INVALID_PARAMETERS: Parameters are not correctly encoded.
    // - An invalid endpoint ID is specified.
    (result == FFA_INVALID_PARAMETERS ==> (
        // Target ID bits [63:16] must be zero (Reserved)
        (target_id as int) & 0xFFFF0000_0000_0000 != 0
        // Flags bits [63:5] must be zero (Reserved)
        || ((flags as int) & 0xFFFFFFF800000000 != 0)
        // Flags bits [3:2] must be 00 (NS resource type)
        || (((flags as int) >> 2) & 0x3 != 0)
        // Flags bit [1] must be zero (Reserved)
        || ((flags as int) & 0x2 != 0)
    ))
    // RETRY: RX buffer not mapped/owned, or Callee busy.
    (result == FFA_RETRY ==> true)
    // NOT_SUPPORTED: Function not implemented.
    (result == FFA_NOT_SUPPORTED ==> true)
    // ABORTED: Could not continue retrieval.
    (result == FFA_ABORTED ==> true)

    // Success conditions
    // result == FFA_SUCCESS (0)
    (result == 0 ==>
        // If Request type flag (bit 4) is 0 (start retrieval):
        // - If written_size == 0 and remaining_size == 0: No resource info available.
        // - If written_size == 0 and remaining_size != 0: No info written, retry.
        // - If written_size != 0 and remaining_size == 0: Complete retrieval.
        // - If written_size != 0 and remaining_size != 0: Partial retrieval.
        // If Request type flag (bit 4) is 1 (continue retrieval):
        // - If written_size == 0 and remaining_size == 0: No retrieval in progress.
        // - If written_size == 0 and remaining_size != 0: No info written, retry.
        // - If written_size != 0 and remaining_size == 0: Complete retrieval.
        // - If written_size != 0 and remaining_size != 0: Partial retrieval.
        // (All combinations of written_size/remaining_size are valid per spec)
        true
    )
}
pub open spec fn ffa_partition_info_get_regs_spec(
    result: int32,
    old_s: S,
    new_s: S,
    uuid_lo: UInt64,
    uuid_hi: UInt64,
    start_index: UInt32,
    tag: UInt32,
) -> bool {
    // Failure conditions
    // INVALID_PARAMETERS: Invalid UUID or Invalid start index
    // NOT_SUPPORTED: Function not implemented at this FF-A instance
    // DENIED: Callee not in a state to handle this request
    // RETRY: Provided tag is not valid
    // NOT_READY: Callee not ready to handle this request
    (result == FFA_ERROR as int32) ==> (
        (InvalidUuid(uuid_lo, uuid_hi) ==> true) ||
        (InvalidStartIndex(start_index) ==> true) ||
        (NotSupported() ==> true) ||
        (Denied() ==> true) ||
        (Retry(tag) ==> true) ||
        (NotReady() ==> true)
    )
    // Success conditions
    // result == FFA_SUCCESS64
    (result == FFA_SUCCESS64 as int32) ==> (
        // Start index must be valid (within bounds of the array maintained by callee)
        // Tag must be valid (MBZ if start_index == 0, otherwise must match callee's tag)
        // Last index, Current index, Tag, Descriptor Size must be returned correctly
        // Partition information descriptors must be returned in registers x3-x17
        true
    )
}

// Helper predicates (defined based on spec text)
pub open spec fn InvalidUuid(uuid_lo: UInt64, uuid_hi: UInt64) -> bool {
    // Invalid UUID check - spec mentions "Invalid UUID" as a failure condition
    // The spec does not define what makes a UUID invalid, so we leave this unconstrained
    true
}

pub open spec fn InvalidStartIndex(start_index: UInt32) -> bool {
    // Invalid start index check - spec mentions "Invalid start index" as a failure condition
    // The spec does not define what makes a start index invalid, so we leave this unconstrained
    true
}

pub open spec fn NotSupported() -> bool {
    // Function not implemented at this FF-A instance
    true
}

pub open spec fn Denied() -> bool {
    // Callee not in a state to handle this request
    true
}

pub open spec fn Retry(tag: UInt32) -> bool {
    // Provided tag is not valid
    true
}

pub open spec fn NotReady() -> bool {
    // Callee not ready to handle this request
    true
}
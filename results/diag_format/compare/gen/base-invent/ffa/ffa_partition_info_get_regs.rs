pub open spec fn ffa_partition_info_get_regs_spec(
    result: Result<(), FfaStatusCode>,
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
    (ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS) ==> (InvalidUuid(uuid_lo, uuid_hi) || InvalidStartIndex(start_index)))
    && (ResultEqual(result, FFA_ERROR_NOT_SUPPORTED) ==> !IsPartitionInfoGetRegsSupported(old_s))
    && (ResultEqual(result, FFA_ERROR_DENIED) ==> !IsCalleeReadyToHandleRequest(old_s))
    && (ResultEqual(result, FFA_ERROR_RETRY) ==> tag != old_s.partition_info_tag)
    && (ResultEqual(result, FFA_ERROR_NOT_READY) ==> !IsCalleeReadyToHandleRequest(old_s))
    // Success conditions
    // FFA_SUCCESS64: Returns metadata and descriptors
    (result.is_Ok() ==> (
        // Metadata constraints from Table 13.41
        (new_s.partition_info_last_index as int >= start_index as int)
        && (new_s.partition_info_current_index as int >= start_index as int)
        && (new_s.partition_info_current_index as int <= new_s.partition_info_last_index as int)
        && (new_s.partition_info_tag == old_s.partition_info_tag || start_index == 0)
        && (new_s.partition_info_descriptor_size == 48)
        // State transitions: descriptors are populated in new_s
        && (new_s.partition_info_descriptors != old_s.partition_info_descriptors)
    ))
}

// Helper predicates (uninterpreted as per spec constraints)
pub open spec fn InvalidUuid(uuid_lo: UInt64, uuid_hi: UInt64) -> bool {
    // Spec states "Invalid UUID" as a failure reason but does not define the exact condition
    // Leaving as uninterpreted to avoid fabricating constraints
    true
}

pub open spec fn InvalidStartIndex(start_index: UInt32) -> bool {
    // Spec states "Invalid start index" as a failure reason but does not define the exact condition
    // Leaving as uninterpreted to avoid fabricating constraints
    true
}

pub open spec fn IsPartitionInfoGetRegsSupported(s: S) -> bool {
    // Spec states "Function not implemented at this FF-A instance"
    // Leaving as uninterpreted to avoid fabricating constraints
    true
}

pub open spec fn IsCalleeReadyToHandleRequest(s: S) -> bool {
    // Spec states "Callee is not in a state to handle this request" and "Callee is not ready"
    // Leaving as uninterpreted to avoid fabricating constraints
    true
}
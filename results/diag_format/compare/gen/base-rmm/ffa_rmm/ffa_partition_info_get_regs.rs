pub open spec fn ffa_partition_info_get_regs_spec(result: UInt32, error_code: Int32, last_index: UInt16, current_index: UInt16, callee_tag: UInt16, desc_size: UInt16, partition_info: [UInt64; 15], old_s: S, new_s: S) -> bool {
    (!IsValidUuid(old_s, uuid_lo, uuid_hi) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidStartIndex(old_s, uuid_lo, uuid_hi, start_index) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET_REGS) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
    && (start_index > 0 && tag != CalleeInfoTag(old_s, uuid_lo, uuid_hi) ==> ResultEqual(result, RETRY))
    && (!CalleeIsReady(old_s) ==> ResultEqual(result, NOT_READY))
    && (ResultEqual(result, FFA_SUCCESS64) ==> (last_index >= start_index && current_index >= start_index && NumEntriesReturned(old_s, last_index, current_index, start_index) == (current_index - start_index) + 1 && callee_tag == CalleeInfoTag(old_s, uuid_lo, uuid_hi) && (!IsNilUuid(old_s, uuid_lo, uuid_hi) ==> DescProtocolUuidFieldsAreZero(old_s, partition_info)) && UnusedRegistersAreZero(old_s, partition_info) && (last_index == current_index ==> AllEntriesReturned(old_s, partition_info))))
}
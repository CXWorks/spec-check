pub open spec fn ffa_ns_res_info_get_spec(target_id: u64, flags: u64, result: Result<(), FfaErrorCode>, length_info: u64, old_s: S, new_s: S) -> bool {
    let target_valid = (flags & 1u64) == 1u64;
    let ns_resource_type = (flags >> 2u64) & 3u64;
    let request_continue = ((flags >> 4u64) & 1u64) == 1u64;
    let sep_id = (target_id & 0xFFFFu64) as u16;
    let params_malformed = (flags >> 5u64) != 0u64
        || ((flags >> 1u64) & 1u64) != 0u64
        || ns_resource_type != 0u64
        || (target_id >> 16u64) != 0u64
        || (!target_valid && target_id != 0u64);
    let endpoint_invalid = target_valid && !IsValidSEndpointId(old_s, sep_id);
    let invalid_params = params_malformed || endpoint_invalid;
    let not_supported = !FfaNsResInfoGetImplemented(old_s);
    let rx_unavailable = !CallerRxBufferMappedInCallee(old_s) || !CallerRxBufferOwnedByCallee(old_s);
    let written_size = length_info >> 32u64;
    let remaining_size = length_info & 0xFFFF_FFFFu64;
    (not_supported ==> (result == Err::<(), FfaErrorCode>(NOT_SUPPORTED) && new_s == old_s))
    && ((!not_supported && invalid_params) ==> (result == Err::<(), FfaErrorCode>(INVALID_PARAMETERS) && new_s == old_s))
    && ((!not_supported && !invalid_params && rx_unavailable) ==> (result == Err::<(), FfaErrorCode>(RETRY) && new_s == old_s))
    && ((!not_supported && !invalid_params && !rx_unavailable && !CalleeIsBusy(old_s) && !(request_continue && RetrievalAborted(old_s))) ==> (
        result.is_Ok()
        && ResourceInfoDescriptorReturnedInRx(old_s, new_s, target_valid, sep_id, request_continue, written_size, remaining_size)
        && ((ns_resource_type == 0u64 && NsPasEntirelyInaccessible(old_s, target_valid, sep_id)) ==> length_info == 0u64)
    ))
}

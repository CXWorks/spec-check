pub open spec fn ffa_partition_info_get_spec(result: FfaReturnCode, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR && (
        (old_s.ff_a_rx_buffer_is_free() ==> ResultEqual(result, FFA_ERROR_BUSY))
        || (old_s.uuid_is_valid() ==> ResultEqual(result, FFA_ERROR_INVALID_PARAMETERS))
        || (old_s.rx_buffer_can_hold_descriptors() ==> ResultEqual(result, FFA_ERROR_NO_MEMORY))
        || (old_s.callee_state_can_handle() ==> ResultEqual(result, FFA_ERROR_DENIED))
        || (old_s.instance_supports_function() ==> ResultEqual(result, FFA_ERROR_NOT_SUPPORTED))
        || (old_s.callee_is_ready() ==> ResultEqual(result, FFA_ERROR_NOT_READY))
    ))
    && (result == FFA_SUCCESS && (
        (old_s.flags_bit0_is_set() ==> (
            new_s.count() >= 1
            && new_s.size() == 0
        ))
        || (old_s.flags_bit0_is_clear() ==> (
            new_s.count() >= 1
            && new_s.size() > 0
            && new_s.descriptors_populated()
        ))
    ))
}
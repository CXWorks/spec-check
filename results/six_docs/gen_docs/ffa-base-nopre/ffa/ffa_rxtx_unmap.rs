pub open spec fn ffa_rxtx_unmap_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> !IsBufferPairRegistered(old_s, old_s.cmd_input_id))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> !IsFfaRxtxUnmapSupported(old_s))
    && (result == FFA_SUCCESS ==> IsBufferPairUnmapped(new_s, old_s.cmd_input_id))
}
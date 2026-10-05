pub open spec fn ffa_rxtx_unmap_spec(result: Result<(), FfaErrorCode>, function_id: UInt32, id: UInt32, old_s: S, new_s: S) -> bool {
    (!FfaRxtxUnmapImplemented(old_s) ==> (result.is_Err() && result.get_Err_0() == NOT_SUPPORTED && new_s == old_s))
    && ((FfaRxtxUnmapImplemented(old_s) && !RxTxBufferPairRegistered(old_s, FfaRxtxUnmapCallerId(old_s, id))) ==> (result.is_Err() && result.get_Err_0() == INVALID_PARAMETERS && new_s == old_s))
    && ((FfaRxtxUnmapImplemented(old_s) && RxTxBufferPairRegistered(old_s, FfaRxtxUnmapCallerId(old_s, id))) ==> (result.is_Ok() && !RxTxBufferPairRegistered(new_s, FfaRxtxUnmapCallerId(old_s, id)) && RxTxBuffersUnchangedExcept(old_s, new_s, FfaRxtxUnmapCallerId(old_s, id))))
}

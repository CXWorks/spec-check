pub open spec fn ffa_rxtx_unmap_spec(result: UInt32, id: UInt16, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_RXTX_UNMAP) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsRxTxBufferPairRegistered(BufferOwner(id)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, FFA_SUCCESS) ==> !IsRxTxBufferPairMappedInCalleeRegime(BufferOwner(id)))
    && (ResultEqual(result, FFA_SUCCESS) ==> RxTxBufferPairMapping(BufferOwner(id)) == old_s.RxTxBufferPairMapping(BufferOwner(id)))
}
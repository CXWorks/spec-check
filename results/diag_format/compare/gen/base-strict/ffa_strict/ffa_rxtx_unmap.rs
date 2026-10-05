pub open spec fn ffa_rxtx_unmap_spec(result: u32, id: u16, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_RXTX_UNMAP, CurrentFfaInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsRxTxBufferPairRegistered(CallerEndpoint(id)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, FFA_SUCCESS) ==> !IsRxTxBufferPairMappedInCalleeRegime(CallerEndpoint(id)))
    && (ResultEqual(result, FFA_SUCCESS) ==> !RxTxBufferPairMappedInCalleeRegime(old_s, CallerEndpoint(id)) || RxTxBufferPairMappedInCalleeRegime(new_s, CallerEndpoint(id)))
}
pub open spec fn ffa_rxtx_unmap_spec(id: UInt16, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_RXTX_UNMAP, CurrentFfaInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRxTxBufferPairRegistered(old_s, CallerEndpoint(old_s, id)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> !IsRxTxBufferPairMappedInCalleeRegime(new_s, CallerEndpoint(new_s, id)))
  && ((IsImplementedAtInstance(old_s, FFA_RXTX_UNMAP, CurrentFfaInstance(old_s)) &&
       IsRxTxBufferPairRegistered(old_s, CallerEndpoint(old_s, id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> IsRxTxBufferPairMappedInCalleeRegime(new_s, CallerEndpoint(new_s, id)))
}
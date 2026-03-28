pub open spec fn rmi_p2p_stream_destroy_spec(stream_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (P2PStreamAt(old_s, stream_ptr).num_pdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, stream_ptr).state == DELEGATED)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       PaIsDelegable(old_s, stream_ptr) &&
       !(GranuleAt(old_s, stream_ptr).state != P2P_STREAM) &&
       !(P2PStreamAt(old_s, stream_ptr).num_pdevs != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, stream_ptr).state == GranuleAt(old_s, stream_ptr).state)
}

pub open spec fn rmi_p2p_stream_create_spec(stream_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, stream_ptr).state == P2P_STREAM)
  && (result.is_Ok() ==> P2PStreamAt(new_s, stream_ptr).num_pdevs == 0)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       PaIsDelegableDram(old_s, stream_ptr) &&
       !(GranuleAt(old_s, stream_ptr).state != DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, stream_ptr).state == GranuleAt(old_s, stream_ptr).state)
  && (result.is_Err()
    ==> P2PStreamAt(new_s, stream_ptr).num_pdevs == P2PStreamAt(old_s, stream_ptr).num_pdevs)
}

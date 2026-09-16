pub open spec fn rmi_pdev_stream_info_spec(pdev_1_ptr: Address, pdev_2_ptr: Address, stream_hnd: Bits64, result: Result<RmiResult, RmiStatusCode>, stream_state: RmiPdevStreamState, stream_type: RmiPdevStreamType, refresh_count: UInt64, purge_count: UInt64, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_1_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, pdev_1_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_1_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).valid != RMM_TRUE ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.stream_type)) && !AddrIsRmiGranuleAligned(old_s, pdev_2_ptr) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.stream_type)) && !PaIsTracked(old_s, pdev_2_ptr) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.stream_type)) && GranuleAt(old_s, pdev_2_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> Equal(stream_state, PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.state))
  && (result.is_Ok() ==> Equal(stream_type, PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.stream_type))
  && (result.is_Ok() && PdevStreamUsesIDE(PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.stream_type) ==> refresh_count == PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.refresh_count)
  && (result.is_Ok() && PdevStreamUsesIDE(PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.stream_type) ==> purge_count == PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.purge_count)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_1_ptr) &&
       PaIsTracked(old_s, pdev_1_ptr) &&
       !(GranuleAt(old_s, pdev_1_ptr).state != GRAN_PDEV) &&
       !(PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).valid != RMM_TRUE) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.stream_type)) && !AddrIsRmiGranuleAligned(old_s, pdev_2_ptr)) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.stream_type)) && !PaIsTracked(old_s, pdev_2_ptr)) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.stream_type)) && GranuleAt(old_s, pdev_2_ptr).state != GRAN_PDEV))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.state == PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.state)
  && (result.is_Err()
    ==> PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.stream_type == PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.stream_type)
  && (result.is_Err()
    ==> PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.refresh_count == PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.refresh_count)
  && (result.is_Err()
    ==> PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),stream_hnd).stream.purge_count == PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),stream_hnd).stream.purge_count)
}
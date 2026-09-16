pub open spec fn rmi_pdev_stream_disconnect_spec(pdev_1_ptr: Address, pdev_2_ptr: Address, stream_hnd: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_1_ptr).state != GRAN_PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_1_ptr).state != PDEV_READY && PdevAt(old_s, pdev_1_ptr).state != PDEV_ERROR) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_1_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).valid != RMM_TRUE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && !AddrIsRmiGranuleAligned(old_s, pdev_2_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && !PaIsTracked(old_s, pdev_2_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && GranuleAt(old_s, pdev_2_ptr).state != GRAN_PDEV) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && (PdevAt(old_s, pdev_2_ptr).state != PDEV_READY && PdevAt(old_s, pdev_2_ptr).state != PDEV_ERROR)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && PdevAt(old_s, pdev_2_ptr).comm_state != DEV_COMM_IDLE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).state != PDEV_STREAM_CONNECTED ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),false).state == PDEV_STREAM_DISCONNECTING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).op == PDEV_OP_DISCONNECT)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() && PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) ==> PdevAt(new_s, pdev_2_ptr).op == PDEV_OP_DISCONNECT)
  && (result.is_Ok() && PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) ==> PdevAt(new_s, pdev_2_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_1_ptr) &&
       PaIsTracked(old_s, pdev_1_ptr) &&
       !(GranuleAt(old_s, pdev_1_ptr).state != GRAN_PDEV) &&
       !((PdevAt(old_s, pdev_1_ptr).state != PDEV_READY && PdevAt(old_s, pdev_1_ptr).state != PDEV_ERROR)) &&
       !(PdevAt(old_s, pdev_1_ptr).comm_state != DEV_COMM_IDLE) &&
       !(PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).valid != RMM_TRUE) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && !AddrIsRmiGranuleAligned(old_s, pdev_2_ptr))) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && !PaIsTracked(old_s, pdev_2_ptr))) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && GranuleAt(old_s, pdev_2_ptr).state != GRAN_PDEV)) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && (PdevAt(old_s, pdev_2_ptr).state != PDEV_READY && PdevAt(old_s, pdev_2_ptr).state != PDEV_ERROR))) &&
       !((PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type) && PdevAt(old_s, pdev_2_ptr).comm_state != DEV_COMM_IDLE)) &&
       !(PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).state != PDEV_STREAM_CONNECTED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevStreamFromHandle(new_s, PdevAt(new_s, pdev_1_ptr), PdevAt(new_s, pdev_2_ptr),false).state == PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).op == PdevAt(old_s, pdev_1_ptr).op)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).comm_state == PdevAt(old_s, pdev_1_ptr).comm_state)
  && (result.is_Err() && PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type)
    ==> PdevAt(new_s, pdev_2_ptr).op == PdevAt(old_s, pdev_2_ptr).op)
  && (result.is_Err() && PdevStreamPdev2Required(old_s, PdevStreamFromHandle(old_s, PdevAt(old_s, pdev_1_ptr), PdevAt(old_s, pdev_2_ptr),false).stream_type)
    ==> PdevAt(new_s, pdev_2_ptr).comm_state == PdevAt(old_s, pdev_2_ptr).comm_state)
}
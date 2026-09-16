pub open spec fn rmi_pdev_stream_connect_spec(params_ptr: Address, result: Result<(), RmiStatusCode>, stream_hnd: Bits64, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!RmiPdevStreamParamsIsValid(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && ((RmiPdevStreamParamsAt(old_s, params_ptr).stream_type == RMI_PDEV_STREAM_NON_TEE && Rmm().static.feat_non_tee_stream != FEATURE_TRUE) ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (!RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.category IN { PDEV_ENDPOINT_ACCEL_OFF_CHIP, PDEV_ENDPOINT_ACCEL_ON_CHIP } ==> result.status == RMI_ERROR_INPUT)
  && (RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.state != PDEV_READY ==> result.status == RMI_ERROR_INPUT)
  && (RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.op != PDEV_OP_NONE ==> result.status == RMI_ERROR_INPUT)
  && (RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.comm_state != DEV_COMM_IDLE ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && !AddrIsRmiGranuleAligned(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2)) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && !PaIsTracked(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2)) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && GranuleAt(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2).state != GRAN_PDEV) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.category != PdevStreamPdev2Category(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type)) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.state != PDEV_READY) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.op != PDEV_OP_NONE) ==> result.status == RMI_ERROR_INPUT)
  && ((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.comm_state != DEV_COMM_IDLE) ==> result.status == RMI_ERROR_INPUT)
  && (PdevStreamFromType(RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1, PdevStreamTypeFromRmi(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type)).valid == RMM_TRUE ==> result.status == RMI_ERROR_INPUT)
  && (PdevStreamAlloc(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2).valid != RMM_TRUE ==> result.status == RMI_ERROR_INPUT)
  && (PdevStreamAlloc(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2).state != PDEV_STREAM_DISCONNECTED ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> stream_hnd == PdevStreamAlloc(new_s, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2).handle)
  && (result.is_Ok() ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1.op == PDEV_OP_CONNECT)
  && (result.is_Ok() ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1.comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() && PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2.op == PDEV_OP_CONNECT)
  && (result.is_Ok() && PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2.comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> PdevStreamAlloc(new_s, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2).state == PDEV_STREAM_CONNECTING)
  && (result.is_Ok() ==> Equal(PdevStreamAlloc(new_s, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2).stream_type, RmiPdevStreamParamsAt(new_s, params_ptr).stream_type))
  && (result.is_Ok() ==> PdevStreamAlloc(new_s, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2).ide_sid == RmiPdevStreamParamsAt(new_s, params_ptr).ide_sid)
  && (result.is_Ok() ==> PdevStreamAlloc(new_s, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2).num_addr_range == RmiPdevStreamParamsAt(new_s, params_ptr).num_addr_range)
  && (result.is_Ok() ==> RmiAddrRangesEqual16(new_s, PdevStreamAlloc(new_s, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2).addr_range, RmiPdevStreamParamsAt(new_s, params_ptr).addr_range, RmiPdevStreamParamsAt(new_s, params_ptr).num_addr_range))
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr) &&
       RmiPdevStreamParamsIsValid(old_s, params_ptr) &&
       !((RmiPdevStreamParamsAt(old_s, params_ptr).stream_type == RMI_PDEV_STREAM_NON_TEE && Rmm().static.feat_non_tee_stream != FEATURE_TRUE)) &&
       AddrIsRmiGranuleAligned(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1) &&
       PaIsTracked(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1) &&
       !(GranuleAt(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1).state != GRAN_PDEV) &&
       RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.category IN { PDEV_ENDPOINT_ACCEL_OFF_CHIP, PDEV_ENDPOINT_ACCEL_ON_CHIP } &&
       !(RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.state != PDEV_READY) &&
       !(RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.op != PDEV_OP_NONE) &&
       !(RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.comm_state != DEV_COMM_IDLE) &&
       !((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && !AddrIsRmiGranuleAligned(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2))) &&
       !((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && !PaIsTracked(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2))) &&
       !((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && GranuleAt(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2).state != GRAN_PDEV)) &&
       !((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.category != PdevStreamPdev2Category(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type))) &&
       !((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.state != PDEV_READY)) &&
       !((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.op != PDEV_OP_NONE)) &&
       !((PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type) && RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.comm_state != DEV_COMM_IDLE)) &&
       !(PdevStreamFromType(RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1, PdevStreamTypeFromRmi(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type)).valid == RMM_TRUE) &&
       !(PdevStreamAlloc(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2).valid != RMM_TRUE) &&
       !(PdevStreamAlloc(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2).state != PDEV_STREAM_DISCONNECTED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1.op == RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.op)
  && (result.is_Err()
    ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1.comm_state == RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1.comm_state)
  && (result.is_Err() && PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type)
    ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2.op == RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.op)
  && (result.is_Err() && PdevStreamPdev2Required(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type)
    ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2.comm_state == RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.comm_state)
  && (result.is_Err()
    ==> PdevStreamAlloc(new_s, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2).state == PdevStreamAlloc(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_1, RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2).state)
  && (!(result.is_Ok() && (RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.category != PdevStreamPdev2Category(old_s, RmiPdevStreamParamsAt(old_s, params_ptr).stream_type))) ==> RmiPdevStreamParamsAt(new_s, params_ptr).pdev_2.category == RmiPdevStreamParamsAt(old_s, params_ptr).pdev_2.category)
}
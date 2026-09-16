pub open spec fn rmi_pdev_create_spec(pdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsPopulatedConventional(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, pdev_ptr) ==> (result.status == RMI_ERROR_TRACKING && result.data.level_addr.level == TrackingToRmiResult(old_s, pdev_ptr) && result.data.level_addr.addr == AddrShiftRmiGranule(old_s, pdev_ptr)))
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_DELEGATED ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!RmiPdevParamsIsValid(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!RmiPdevFlagsSupported(old_s, params.flags) ==> result.status == RMI_ERROR_INPUT)
  && (((params.rid_top - params.rid_base) > ((2 ^ Rmm().static.pdev_max_vdevs_order) - 1)) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> GranuleAt(new_s, pdev_ptr).state == GRAN_PDEV)
  && (result.is_Ok() ==> Equal(PdevAt(new_s, pdev_ptr).category, params.flags.category))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).hb_base == params.hb_base)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).pdev_id == params.pdev_id)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).routing_id == params.routing_id)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).rid_base == params.rid_base)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).rid_top == params.rid_top)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).id_index == params.id_index)
  && (result.is_Ok() ==> Equal(PdevAt(new_s, pdev_ptr).hash_algo, params.hash_algo))
  && (result.is_Ok() ==> Equal(PdevAt(new_s, pdev_ptr).spdm, params.flags.spdm))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).op == PDEV_OP_NONE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).max_num_vdevs == params.rid_top - params.rid_base)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == 0)
  && (result.is_Ok() ==> !PdevStreamLive(new_s, PdevAt(new_s, pdev_ptr)))
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsPopulatedConventional(old_s, pdev_ptr) &&
       PaIsTrackedFine(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_DELEGATED) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr) &&
       RmiPdevParamsIsValid(old_s, params_ptr) &&
       RmiPdevFlagsSupported(old_s, params.flags) &&
       !(((params.rid_top - params.rid_base) > ((2 ^ Rmm().static.pdev_max_vdevs_order) - 1))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, pdev_ptr).state == GranuleAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}
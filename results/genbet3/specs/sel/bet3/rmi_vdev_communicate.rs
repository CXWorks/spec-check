pub open spec fn rmi_vdev_communicate_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, data_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result == Err(RMI_ERROR_NOT_SUPPORTED(0)))
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result == Err(RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> result == Err(RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result == Err(RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result == Err(RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, pdev_ptr) ==> result == Err(RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result == Err(RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> result == Err(RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, vdev_ptr) ==> result == Err(RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> result == Err(RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> result == Err(RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, data_ptr) ==> result == Err(RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, data_ptr) ==> result == Err(RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, data_ptr, 0 as int) ==> result == Err(RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, data_ptr, 0 as int) ==> result == Err(RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, data.enter.req_addr) ==> result == Err(RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, data.enter.req_addr) ==> result == Err(RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, data.enter.rsp_addr) ==> result == Err(RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, data.enter.rsp_addr) ==> result == Err(RMI_ERROR_INPUT))
  && (data.enter.rsp_len > Rmm().dynamic.rmi_granule_size ==> result == Err(RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> result == Err(RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE ==> result == Err(RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).op == VDEV_OP_LOCK && last_lock_seq_pre(old_s, RealmAt(old_s, rd), VdevAt(old_s, vdev_ptr).vdev_id) == UINT64_MAX ==> result == Err(RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).op == VDEV_OP_GET_MEAS && VdevAt(old_s, vdev_ptr).freshness.meas_seq == UINT64_MAX ==> result == Err(RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).op == VDEV_OP_GET_REPORT && VdevAt(old_s, vdev_ptr).freshness.report_seq == UINT64_MAX ==> result == Err(RMI_ERROR_DEVICE))
  && (PdevIsBusy(old_s, PdevAt(old_s, pdev_ptr)) ==> result == Err(RSI_BUSY))
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) != DEV_COMM_ERROR && !VdevUnlockKeyRefreshCheck(new_s, VdevAt(new_s, vdev_ptr), data) && !(VdevUnlockKeyPurgeCheck(new_s, VdevAt(new_s, vdev_ptr), data) && !VdevStreamPurgeComplete(new_s, VdevAt(new_s, vdev_ptr)))) ==> VdevAt(new_s, vdev_ptr).comm_state == DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data))
  && (result.is_Ok() && DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_ERROR ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
  && (result.is_Ok() && DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_ERROR ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW && VdevAt(new_s, vdev_ptr).op == VDEV_OP_INIT) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_UNLOCKED)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && (VdevAt(new_s, vdev_ptr).vdev_state == VDEV_LOCKED || VdevAt(new_s, vdev_ptr).vdev_state == VDEV_STARTED || VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR) && VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_UNLOCKED)
  && (result.is_Ok() && (VdevUnlockKeyRefreshCheck(old_s, VdevAt(old_s, vdev_ptr), data) && VdevStreamRefreshComplete(old_s, VdevAt(old_s, vdev_ptr))) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_KEY_PURGE)
  && (result.is_Ok() && (VdevUnlockKeyRefreshCheck(old_s, VdevAt(old_s, vdev_ptr), data) && VdevStreamRefreshComplete(old_s, VdevAt(old_s, vdev_ptr))) ==> VdevAt(new_s, vdev_ptr).stream_purge_cnt == VdevStreamPurgeCountSnapshot(new_s, VdevAt(new_s, vdev_ptr)))
  && (result.is_Ok() && (VdevUnlockKeyRefreshCheck(old_s, VdevAt(old_s, vdev_ptr), data) && !VdevStreamRefreshComplete(old_s, VdevAt(old_s, vdev_ptr))) ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() && (VdevUnlockKeyPurgeCheck(old_s, VdevAt(old_s, vdev_ptr), data) && VdevStreamPurgeComplete(old_s, VdevAt(old_s, vdev_ptr))) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_UNLOCKED)
  && (result.is_Ok() && (VdevUnlockKeyPurgeCheck(old_s, VdevAt(old_s, vdev_ptr), data) && !VdevStreamPurgeComplete(old_s, VdevAt(old_s, vdev_ptr))) ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_LOCKED)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK) ==> VdevAt(new_s, vdev_ptr).started_once == RMM_FALSE)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK) ==> VdevAt(new_s, vdev_ptr).freshness.lock_seq == last_lock_seq_pre(new_s, RealmAt(new_s, rd), VdevAt(new_s, vdev_ptr).vdev_id) + 1)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK) ==> VdevLastLockSequence(new_s, RealmAt(new_s, rd), VdevAt(new_s, vdev_ptr).vdev_id) == last_lock_seq_pre(new_s, RealmAt(new_s, rd), VdevAt(new_s, vdev_ptr).vdev_id) + 1)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_MEAS) ==> VdevAt(new_s, vdev_ptr).freshness.meas_seq == VdevAt(new_s, vdev_ptr).freshness.meas_seq + 1)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT) ==> VdevAt(new_s, vdev_ptr).freshness.report_seq == VdevAt(new_s, vdev_ptr).freshness.report_seq + 1)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && !VdevUnlockKeyRefreshCheck(new_s, VdevAt(new_s, vdev_ptr), data) && !(VdevUnlockKeyPurgeCheck(new_s, VdevAt(new_s, vdev_ptr), data) && !VdevStreamPurgeComplete(new_s, VdevAt(new_s, vdev_ptr)))) ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_NONE)
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && !VdevUnlockKeyRefreshCheck(new_s, VdevAt(new_s, vdev_ptr), data) && !(VdevUnlockKeyPurgeCheck(new_s, VdevAt(new_s, vdev_ptr), data) && !VdevStreamPurgeComplete(new_s, VdevAt(new_s, vdev_ptr)))) ==> RmiDevCommComplete(new_s, data.exit.flags))
  && (result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) != DEV_COMM_IDLE || VdevUnlockKeyRefreshCheck(new_s, VdevAt(new_s, vdev_ptr), data) || (VdevUnlockKeyPurgeCheck(new_s, VdevAt(new_s, vdev_ptr), data) && !VdevStreamPurgeComplete(new_s, VdevAt(new_s, vdev_ptr)))) ==> !RmiDevCommComplete(new_s, data.exit.flags))
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsTracked(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       AddrIsRmiGranuleAligned(old_s, data_ptr) &&
       NonSecureAccessPermitted(old_s, data_ptr) &&
       AddrIsRmiGranuleAligned(old_s, data_ptr, 0 as int) &&
       NonSecureAccessPermitted(old_s, data_ptr, 0 as int) &&
       AddrIsRmiGranuleAligned(old_s, data.enter.req_addr) &&
       NonSecureAccessPermitted(old_s, data.enter.req_addr) &&
       AddrIsRmiGranuleAligned(old_s, data.enter.rsp_addr) &&
       NonSecureAccessPermitted(old_s, data.enter.rsp_addr) &&
       !(data.enter.rsp_len > Rmm().dynamic.rmi_granule_size) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !(VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE) &&
       !(VdevAt(old_s, vdev_ptr).op == VDEV_OP_LOCK && last_lock_seq_pre(old_s, RealmAt(old_s, rd), VdevAt(old_s, vdev_ptr).vdev_id) == UINT64_MAX) &&
       !(VdevAt(old_s, vdev_ptr).op == VDEV_OP_GET_MEAS && VdevAt(old_s, vdev_ptr).freshness.meas_seq == UINT64_MAX) &&
       !(VdevAt(old_s, vdev_ptr).op == VDEV_OP_GET_REPORT && VdevAt(old_s, vdev_ptr).freshness.report_seq == UINT64_MAX) &&
       !(PdevIsBusy(old_s, PdevAt(old_s, pdev_ptr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).stream_purge_cnt == VdevAt(old_s, vdev_ptr).stream_purge_cnt)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (!(result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK)) ==> VdevAt(new_s, vdev_ptr).freshness.meas_seq == VdevAt(old_s, vdev_ptr).freshness.meas_seq)
  && (!(result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT)) ==> VdevAt(new_s, vdev_ptr).freshness.report_seq == VdevAt(old_s, vdev_ptr).freshness.report_seq)
  && (!(result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK)) ==> VdevAt(new_s, vdev_ptr).freshness.report_seq == VdevAt(old_s, vdev_ptr).freshness.report_seq)
  && (!(result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK)) ==> VdevAt(new_s, vdev_ptr).freshness.meas_seq == VdevAt(old_s, vdev_ptr).freshness.meas_seq)
  && (!(result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK)) ==> VdevAt(new_s, vdev_ptr).freshness.lock_seq == VdevAt(old_s, vdev_ptr).freshness.lock_seq)
  && (!(result.is_Ok() && (DeviceCommunicate(new_s, VdevAt(new_s, vdev_ptr), data) == DEV_COMM_IDLE && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK)) ==> VdevAt(new_s, vdev_ptr).started_once == VdevAt(old_s, vdev_ptr).started_once)
}
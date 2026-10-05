pub open spec fn hci_volume_p2p_bind_spec(stream_ptr: Address, vd: Address, worker_ptr: Address, drive_1_ptr: Address, drive_2_ptr: Address, volume_1_ptr: Address, volume_2_ptr: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_dx != FEATURE_TRUE ==> result == HCI_ERROR_NOT_SUPPORTED)
  && (vd % extent_size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD ==> result == HCI_ERROR_INPUT)
  && (worker_ptr % extent_size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, worker_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, worker_ptr).state != WORKER ==> result == HCI_ERROR_INPUT)
  && (Workerat(old_s, worker_ptr).state == WORKER_RUNNING ==> result == HCI_ERROR_WORKER)
  && (Workerat(old_s, worker_ptr).vault != Vaultat(old_s, vd) ==> result == HCI_ERROR_WORKER)
  && (stream_ptr % extent_size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, stream_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, stream_ptr).state != P2P_STREAM ==> result == HCI_ERROR_INPUT)
  && (drive_1_ptr % extent_size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, drive_1_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, drive_1_ptr).state != DRIVE ==> result == HCI_ERROR_INPUT)
  && ((Driveat(old_s, drive_1_ptr).p2p_stream_valid != KEEPER_TRUE || Driveat(old_s, drive_1_ptr).p2p_stream != stream_ptr) ==> result == HCI_ERROR_INPUT)
  && (drive_2_ptr % extent_size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, drive_2_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, drive_2_ptr).state != DRIVE ==> result == HCI_ERROR_INPUT)
  && ((Driveat(old_s, drive_2_ptr).p2p_stream_valid != KEEPER_TRUE || Driveat(old_s, drive_2_ptr).p2p_stream != stream_ptr) ==> result == HCI_ERROR_INPUT)
  && (volume_1_ptr % extent_size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, volume_1_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, volume_1_ptr).state != VOLUME ==> result == HCI_ERROR_INPUT)
  && (Volumeat(old_s, volume_1_ptr).vault != Vaultat(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (Volumeat(old_s, volume_1_ptr).drive != drive_1_ptr ==> result == HCI_ERROR_INPUT)
  && (Volumeat(old_s, volume_1_ptr).comm_state != DEV_COMM_IDLE ==> result == HCI_ERROR_DEVICE)
  && (Volumeat(old_s, volume_1_ptr).volume_certify_info_1 != Workerat(old_s, worker_ptr).volume_certify_info_1 ==> result == HCI_ERROR_DEVICE)
  && (Volumeat(old_s, volume_1_ptr).p2p_bound != FEATURE_FALSE ==> result == HCI_ERROR_DEVICE)
  && (volume_2_ptr % extent_size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, volume_2_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, volume_2_ptr).state != VOLUME ==> result == HCI_ERROR_INPUT)
  && (Volumeat(old_s, volume_2_ptr).vault != Vaultat(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (Volumeat(old_s, volume_2_ptr).drive != drive_2_ptr ==> result == HCI_ERROR_INPUT)
  && (Volumeat(old_s, volume_2_ptr).comm_state != DEV_COMM_IDLE ==> result == HCI_ERROR_DEVICE)
  && (Volumeat(old_s, volume_2_ptr).volume_certify_info_2 != Workerat(old_s, worker_ptr).volume_certify_info_2 ==> result == HCI_ERROR_DEVICE)
  && (Volumeat(old_s, volume_2_ptr).p2p_bound != FEATURE_FALSE ==> result == HCI_ERROR_DEVICE)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_1_ptr).op == VOLUME_OP_P2P_BIND)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_1_ptr).p2p_bound == FEATURE_TRUE)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_1_ptr).p2p_stream == Peerstreamat(new_s, stream_ptr))
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_1_ptr).p2p_peer == Volumeat(new_s, volume_2_ptr).id)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_2_ptr).op == VOLUME_OP_P2P_BIND)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_2_ptr).comm_state == DEV_COMM_PENDING)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_2_ptr).p2p_bound == FEATURE_TRUE)
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_2_ptr).p2p_stream == Peerstreamat(new_s, stream_ptr))
  && (result == HCI_SUCCESS ==> Volumeat(new_s, volume_2_ptr).p2p_peer == Volumeat(new_s, volume_1_ptr).id)
  && ((!(ImplFeatures(old_s).feat_dx != FEATURE_TRUE) &&
       vd % extent_size(old_s) == 0 &&
       is_enrollable_physical_address(old_s, vd) &&
       ExtentAt(old_s, vd).state == VD &&
       worker_ptr % extent_size(old_s) == 0 &&
       is_enrollable_physical_address(old_s, worker_ptr) &&
       ExtentAt(old_s, worker_ptr).state == WORKER &&
       !(Workerat(old_s, worker_ptr).state == WORKER_RUNNING) &&
       Workerat(old_s, worker_ptr).vault == Vaultat(old_s, vd) &&
       stream_ptr % extent_size(old_s) == 0 &&
       is_enrollable_physical_address(old_s, stream_ptr) &&
       ExtentAt(old_s, stream_ptr).state == P2P_STREAM &&
       drive_1_ptr % extent_size(old_s) == 0 &&
       is_enrollable_physical_address(old_s, drive_1_ptr) &&
       ExtentAt(old_s, drive_1_ptr).state == DRIVE &&
       !((Driveat(old_s, drive_1_ptr).p2p_stream_valid != KEEPER_TRUE || Driveat(old_s, drive_1_ptr).p2p_stream != stream_ptr)) &&
       drive_2_ptr % extent_size(old_s) == 0 &&
       is_enrollable_physical_address(old_s, drive_2_ptr) &&
       ExtentAt(old_s, drive_2_ptr).state == DRIVE &&
       !((Driveat(old_s, drive_2_ptr).p2p_stream_valid != KEEPER_TRUE || Driveat(old_s, drive_2_ptr).p2p_stream != stream_ptr)) &&
       volume_1_ptr % extent_size(old_s) == 0 &&
       is_enrollable_physical_address(old_s, volume_1_ptr) &&
       ExtentAt(old_s, volume_1_ptr).state == VOLUME &&
       Volumeat(old_s, volume_1_ptr).vault == Vaultat(old_s, vd) &&
       Volumeat(old_s, volume_1_ptr).drive == drive_1_ptr &&
       Volumeat(old_s, volume_1_ptr).comm_state == DEV_COMM_IDLE &&
       Volumeat(old_s, volume_1_ptr).volume_certify_info_1 == Workerat(old_s, worker_ptr).volume_certify_info_1 &&
       Volumeat(old_s, volume_1_ptr).p2p_bound == FEATURE_FALSE &&
       volume_2_ptr % extent_size(old_s) == 0 &&
       is_enrollable_physical_address(old_s, volume_2_ptr) &&
       ExtentAt(old_s, volume_2_ptr).state == VOLUME &&
       Volumeat(old_s, volume_2_ptr).vault == Vaultat(old_s, vd) &&
       Volumeat(old_s, volume_2_ptr).drive == drive_2_ptr &&
       Volumeat(old_s, volume_2_ptr).comm_state == DEV_COMM_IDLE &&
       Volumeat(old_s, volume_2_ptr).volume_certify_info_2 == Workerat(old_s, worker_ptr).volume_certify_info_2 &&
       Volumeat(old_s, volume_2_ptr).p2p_bound == FEATURE_FALSE)
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_1_ptr).op == Volumeat(old_s, volume_1_ptr).op)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_1_ptr).comm_state == Volumeat(old_s, volume_1_ptr).comm_state)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_1_ptr).p2p_bound == Volumeat(old_s, volume_1_ptr).p2p_bound)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_1_ptr).p2p_stream == Volumeat(old_s, volume_1_ptr).p2p_stream)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_1_ptr).p2p_peer == Volumeat(old_s, volume_1_ptr).p2p_peer)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_2_ptr).op == Volumeat(old_s, volume_2_ptr).op)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_2_ptr).comm_state == Volumeat(old_s, volume_2_ptr).comm_state)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_2_ptr).p2p_bound == Volumeat(old_s, volume_2_ptr).p2p_bound)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_2_ptr).p2p_stream == Volumeat(old_s, volume_2_ptr).p2p_stream)
  && (result != HCI_SUCCESS
    ==> Volumeat(new_s, volume_2_ptr).p2p_peer == Volumeat(old_s, volume_2_ptr).p2p_peer)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_1_ptr).op == Volumeat(old_s, volume_1_ptr).op)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_1_ptr).comm_state == Volumeat(old_s, volume_1_ptr).comm_state)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_1_ptr).p2p_bound == Volumeat(old_s, volume_1_ptr).p2p_bound)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_1_ptr).p2p_stream == Volumeat(old_s, volume_1_ptr).p2p_stream)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_1_ptr).p2p_peer == Volumeat(old_s, volume_1_ptr).p2p_peer)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_2_ptr).op == Volumeat(old_s, volume_2_ptr).op)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_2_ptr).comm_state == Volumeat(old_s, volume_2_ptr).comm_state)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_2_ptr).p2p_bound == Volumeat(old_s, volume_2_ptr).p2p_bound)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_2_ptr).p2p_stream == Volumeat(old_s, volume_2_ptr).p2p_stream)
  && (result == HCI_ERROR_NOT_SUPPORTED
    ==> Volumeat(new_s, volume_2_ptr).p2p_peer == Volumeat(old_s, volume_2_ptr).p2p_peer)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_1_ptr).op == Volumeat(old_s, volume_1_ptr).op)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_1_ptr).comm_state == Volumeat(old_s, volume_1_ptr).comm_state)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_1_ptr).p2p_bound == Volumeat(old_s, volume_1_ptr).p2p_bound)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_1_ptr).p2p_stream == Volumeat(old_s, volume_1_ptr).p2p_stream)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_1_ptr).p2p_peer == Volumeat(old_s, volume_1_ptr).p2p_peer)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_2_ptr).op == Volumeat(old_s, volume_2_ptr).op)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_2_ptr).comm_state == Volumeat(old_s, volume_2_ptr).comm_state)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_2_ptr).p2p_bound == Volumeat(old_s, volume_2_ptr).p2p_bound)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_2_ptr).p2p_stream == Volumeat(old_s, volume_2_ptr).p2p_stream)
  && (result == HCI_ERROR_INPUT
    ==> Volumeat(new_s, volume_2_ptr).p2p_peer == Volumeat(old_s, volume_2_ptr).p2p_peer)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_1_ptr).op == Volumeat(old_s, volume_1_ptr).op)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_1_ptr).comm_state == Volumeat(old_s, volume_1_ptr).comm_state)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_1_ptr).p2p_bound == Volumeat(old_s, volume_1_ptr).p2p_bound)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_1_ptr).p2p_stream == Volumeat(old_s, volume_1_ptr).p2p_stream)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_1_ptr).p2p_peer == Volumeat(old_s, volume_1_ptr).p2p_peer)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_2_ptr).op == Volumeat(old_s, volume_2_ptr).op)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_2_ptr).comm_state == Volumeat(old_s, volume_2_ptr).comm_state)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_2_ptr).p2p_bound == Volumeat(old_s, volume_2_ptr).p2p_bound)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_2_ptr).p2p_stream == Volumeat(old_s, volume_2_ptr).p2p_stream)
  && (result == HCI_ERROR_WORKER
    ==> Volumeat(new_s, volume_2_ptr).p2p_peer == Volumeat(old_s, volume_2_ptr).p2p_peer)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_1_ptr).op == Volumeat(old_s, volume_1_ptr).op)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_1_ptr).comm_state == Volumeat(old_s, volume_1_ptr).comm_state)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_1_ptr).p2p_bound == Volumeat(old_s, volume_1_ptr).p2p_bound)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_1_ptr).p2p_stream == Volumeat(old_s, volume_1_ptr).p2p_stream)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_1_ptr).p2p_peer == Volumeat(old_s, volume_1_ptr).p2p_peer)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_2_ptr).op == Volumeat(old_s, volume_2_ptr).op)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_2_ptr).comm_state == Volumeat(old_s, volume_2_ptr).comm_state)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_2_ptr).p2p_bound == Volumeat(old_s, volume_2_ptr).p2p_bound)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_2_ptr).p2p_stream == Volumeat(old_s, volume_2_ptr).p2p_stream)
  && (result == HCI_ERROR_DEVICE
    ==> Volumeat(new_s, volume_2_ptr).p2p_peer == Volumeat(old_s, volume_2_ptr).p2p_peer)
}
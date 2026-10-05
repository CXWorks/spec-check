pub open spec fn hci_volume_p2p_bind_spec(vd: Address, worker_ptr: Address, stream_ptr: Address, drive_1_ptr: Address, drive_2_ptr: Address, volume_1_ptr: Address, volume_2_ptr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_dx != FEATURE_TRUE ==> result == HCI_ERROR_NOT_SUPPORTED)
  && (vd % size_of::<Extent>() != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && (worker_ptr % size_of::<Extent>() != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, worker_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, worker_ptr).state != WORKER_STATE ==> result == HCI_ERROR_INPUT)
  && (WorkerAt(old_s, worker_ptr).state == WORKER_RUNNING ==> result == HCI_ERROR_WORKER)
  && (WorkerAt(old_s, worker_ptr).owner_vd != vd ==> result == HCI_ERROR_WORKER)
  && (stream_ptr % size_of::<Extent>() != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, stream_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, stream_ptr).state != P2P_STREAM_STATE ==> result == HCI_ERROR_INPUT)
  && (drive_1_ptr % size_of::<Extent>() != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, drive_1_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, drive_1_ptr).state != DRIVE_STATE ==> result == HCI_ERROR_INPUT)
  && (DriveAt(old_s, drive_1_ptr).p2p_stream_valid != KEEPER_TRUE ==> result == HCI_ERROR_INPUT)
  && (DriveAt(old_s, drive_1_ptr).p2p_stream != stream_ptr ==> result == HCI_ERROR_INPUT)
  && (drive_2_ptr % size_of::<Extent>() != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, drive_2_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, drive_2_ptr).state != DRIVE_STATE ==> result == HCI_ERROR_INPUT)
  && (DriveAt(old_s, drive_2_ptr).p2p_stream_valid != KEEPER_TRUE ==> result == HCI_ERROR_INPUT)
  && (DriveAt(old_s, drive_2_ptr).p2p_stream != stream_ptr ==> result == HCI_ERROR_INPUT)
  && (volume_1_ptr % size_of::<Extent>() != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, volume_1_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, volume_1_ptr).state != VOLUME_STATE ==> result == HCI_ERROR_INPUT)
  && (VolumeAt(old_s, volume_1_ptr).vault_vd != vd ==> result == HCI_ERROR_INPUT)
  && (VolumeAt(old_s, volume_1_ptr).drive != drive_1_ptr ==> result == HCI_ERROR_INPUT)
  && (VolumeAt(old_s, volume_1_ptr).comm_state != DEV_COMM_IDLE ==> result == HCI_ERROR_DEVICE)
  && (VolumeAt(old_s, volume_1_ptr).volume_certify_info_1 != WorkerAt(old_s, worker_ptr).volume_certify_info_1 ==> result == HCI_ERROR_DEVICE)
  && (VolumeAt(old_s, volume_1_ptr).p2p_bound != FEATURE_FALSE ==> result == HCI_ERROR_DEVICE)
  && (volume_2_ptr % size_of::<Extent>() != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, volume_2_ptr) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, volume_2_ptr).state != VOLUME_STATE ==> result == HCI_ERROR_INPUT)
  && (VolumeAt(old_s, volume_2_ptr).vault_vd != vd ==> result == HCI_ERROR_INPUT)
  && (VolumeAt(old_s, volume_2_ptr).drive != drive_2_ptr ==> result == HCI_ERROR_INPUT)
  && (VolumeAt(old_s, volume_2_ptr).comm_state != DEV_COMM_IDLE ==> result == HCI_ERROR_DEVICE)
  && (VolumeAt(old_s, volume_2_ptr).volume_certify_info_2 != WorkerAt(old_s, worker_ptr).volume_certify_info_2 ==> result == HCI_ERROR_DEVICE)
  && (VolumeAt(old_s, volume_2_ptr).p2p_bound != FEATURE_FALSE ==> result == HCI_ERROR_DEVICE)
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_1_ptr).operation == VOLUME_OP_P2P_BIND)
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_1_ptr).p2p_bound == FEATURE_TRUE)
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_1_ptr).p2p_stream == P2P_STREAMAt(new_s, stream_ptr))
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_1_ptr).p2p_peer == VolumeIdentifier(new_s, VolumeAt(new_s, volume_2_ptr)))
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_2_ptr).operation == VOLUME_OP_P2P_BIND)
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_2_ptr).comm_state == DEV_COMM_PENDING)
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_2_ptr).p2p_bound == FEATURE_TRUE)
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_2_ptr).p2p_stream == P2P_STREAMAt(new_s, stream_ptr))
  && (result == RSI_SUCCESS ==> VolumeAt(new_s, volume_2_ptr).p2p_peer == VolumeIdentifier(new_s, VolumeAt(new_s, volume_1_ptr)))
  && ((!(ImplFeatures(old_s).feat_dx != FEATURE_TRUE) &&
       vd % size_of::<Extent>() == 0 &&
       is_enrollable_physical_address(old_s, vd) &&
       ExtentAt(old_s, vd).state == VD_STATE &&
       worker_ptr % size_of::<Extent>() == 0 &&
       is_enrollable_physical_address(old_s, worker_ptr) &&
       ExtentAt(old_s, worker_ptr).state == WORKER_STATE &&
       !(WorkerAt(old_s, worker_ptr).state == WORKER_RUNNING) &&
       WorkerAt(old_s, worker_ptr).owner_vd == vd &&
       stream_ptr % size_of::<Extent>() == 0 &&
       is_enrollable_physical_address(old_s, stream_ptr) &&
       ExtentAt(old_s, stream_ptr).state == P2P_STREAM_STATE &&
       drive_1_ptr % size_of::<Extent>() == 0 &&
       is_enrollable_physical_address(old_s, drive_1_ptr) &&
       ExtentAt(old_s, drive_1_ptr).state == DRIVE_STATE &&
       !(DriveAt(old_s, drive_1_ptr).p2p_stream_valid != KEEPER_TRUE) &&
       !(DriveAt(old_s, drive_1_ptr).p2p_stream != stream_ptr) &&
       drive_2_ptr % size_of::<Extent>() == 0 &&
       is_enrollable_physical_address(old_s, drive_2_ptr) &&
       ExtentAt(old_s, drive_2_ptr).state == DRIVE_STATE &&
       !(DriveAt(old_s, drive_2_ptr).p2p_stream_valid != KEEPER_TRUE) &&
       !(DriveAt(old_s, drive_2_ptr).p2p_stream != stream_ptr) &&
       volume_1_ptr % size_of::<Extent>() == 0 &&
       is_enrollable_physical_address(old_s, volume_1_ptr) &&
       ExtentAt(old_s, volume_1_ptr).state == VOLUME_STATE &&
       VolumeAt(old_s, volume_1_ptr).vault_vd == vd &&
       VolumeAt(old_s, volume_1_ptr).drive == drive_1_ptr &&
       !(VolumeAt(old_s, volume_1_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VolumeAt(old_s, volume_1_ptr).volume_certify_info_1 != WorkerAt(old_s, worker_ptr).volume_certify_info_1) &&
       !(VolumeAt(old_s, volume_1_ptr).p2p_bound != FEATURE_FALSE) &&
       volume_2_ptr % size_of::<Extent>() == 0 &&
       is_enrollable_physical_address(old_s, volume_2_ptr) &&
       ExtentAt(old_s, volume_2_ptr).state == VOLUME_STATE &&
       VolumeAt(old_s, volume_2_ptr).vault_vd == vd &&
       VolumeAt(old_s, volume_2_ptr).drive == drive_2_ptr &&
       !(VolumeAt(old_s, volume_2_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VolumeAt(old_s, volume_2_ptr).volume_certify_info_2 != WorkerAt(old_s, worker_ptr).volume_certify_info_2) &&
       !(VolumeAt(old_s, volume_2_ptr).p2p_bound != FEATURE_FALSE))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_1_ptr).operation == VolumeAt(old_s, volume_1_ptr).operation)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_1_ptr).comm_state == VolumeAt(old_s, volume_1_ptr).comm_state)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_1_ptr).p2p_bound == VolumeAt(old_s, volume_1_ptr).p2p_bound)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_1_ptr).p2p_stream == VolumeAt(old_s, volume_1_ptr).p2p_stream)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_1_ptr).p2p_peer == VolumeAt(old_s, volume_1_ptr).p2p_peer)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_2_ptr).operation == VolumeAt(old_s, volume_2_ptr).operation)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_2_ptr).comm_state == VolumeAt(old_s, volume_2_ptr).comm_state)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_2_ptr).p2p_bound == VolumeAt(old_s, volume_2_ptr).p2p_bound)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_2_ptr).p2p_stream == VolumeAt(old_s, volume_2_ptr).p2p_stream)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_2_ptr).p2p_peer == VolumeAt(old_s, volume_2_ptr).p2p_peer)
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_1_ptr).p2p_stream == P2P_STREAMAt(new_s, stream_ptr))
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_1_ptr).p2p_peer == VolumeIdentifier(new_s, VolumeAt(new_s, volume_2_ptr)))
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_2_ptr).p2p_stream == P2P_STREAMAt(new_s, stream_ptr))
  && (result != RSI_SUCCESS
    ==> VolumeAt(new_s, volume_2_ptr).p2p_peer == VolumeIdentifier(new_s, VolumeAt(new_s, volume_1_ptr)))
}
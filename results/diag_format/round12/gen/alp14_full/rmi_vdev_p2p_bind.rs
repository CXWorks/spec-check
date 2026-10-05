pub open spec fn rmi_vdev_p2p_bind_spec(stream_ptr: Address, rd: Address, rec_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, vdev_1_ptr: Address, vdev_2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (is_aligned_to_granule_size(old_s, rd) ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, rd) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state != RD ==> result.is_Ok())
  && (is_aligned_to_granule_size(old_s, rec_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, rec_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, rec_ptr).state != REC ==> result.is_Ok())
  && (RealmAt(old_s, rd).rec_state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RealmAt(old_s, rd).owner != rec_ptr ==> ResultEqual(result, RMI_ERROR_REC))
  && (is_aligned_to_granule_size(old_s, stream_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, stream_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> result.is_Ok())
  && (is_aligned_to_granule_size(old_s, pdev_1_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, pdev_1_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> result.is_Ok())
  && (PDEVAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PDEVAt(old_s, pdev_1_ptr).p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_aligned_to_granule_size(old_s, pdev_2_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, pdev_2_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> result.is_Ok())
  && (PDEVAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PDEVAt(old_s, pdev_2_ptr).p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_aligned_to_granule_size(old_s, vdev_1_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, vdev_1_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, vdev_1_ptr).state != VDEV ==> result.is_Ok())
  && (VDEVAt(old_s, vdev_1_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VDEVAt(old_s, vdev_1_ptr).pdev != pdev_1_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VDEVAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VDEVAt(old_s, vdev_1_ptr).vdev_attest_info_1 != VDEVAt(old_s, rec_ptr).vdev_attest_info_1 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VDEVAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (is_aligned_to_granule_size(old_s, vdev_2_ptr) ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, vdev_2_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, vdev_2_ptr).state != VDEV ==> result.is_Ok())
  && (VDEVAt(old_s, vdev_2_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VDEVAt(old_s, vdev_2_ptr).pdev != pdev_2_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VDEVAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VDEVAt(old_s, vdev_2_ptr).vdev_attest_info_2 != VDEVAt(old_s, rec_ptr).vdev_attest_info_2 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VDEVAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_1_ptr).operation == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_1_ptr).p2p_bound == FEATURE_TRUE)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_1_ptr).p2p_stream == stream_ptr)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_1_ptr).p2p_peer == VDEVAt(new_s, vdev_2_ptr).vdev_id)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_2_ptr).operation == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_2_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_2_ptr).p2p_bound == FEATURE_TRUE)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_2_ptr).p2p_stream == stream_ptr)
  && (result.is_Ok() ==> VDEVAt(new_s, vdev_2_ptr).p2p_peer == VDEVAt(new_s, vdev_1_ptr).vdev_id)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       is_aligned_to_granule_size(old_s, rd) &&
       is_delegable_physical_address(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       is_aligned_to_granule_size(old_s, rec_ptr) &&
       is_delegable_physical_address(old_s, rec_ptr) &&
       GranuleAt(old_s, rec_ptr).state == REC &&
       !(RealmAt(old_s, rd).rec_state == REC_RUNNING) &&
       RealmAt(old_s, rd).owner == rec_ptr &&
       is_aligned_to_granule_size(old_s, stream_ptr) &&
       is_delegable_physical_address(old_s, stream_ptr) &&
       GranuleAt(old_s, stream_ptr).state == P2P_STREAM &&
       is_aligned_to_granule_size(old_s, pdev_1_ptr) &&
       is_delegable_physical_address(old_s, pdev_1_ptr) &&
       GranuleAt(old_s, pdev_1_ptr).state == PDEV &&
       !(PDEVAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE) &&
       !(PDEVAt(old_s, pdev_1_ptr).p2p_stream != stream_ptr) &&
       is_aligned_to_granule_size(old_s, pdev_2_ptr) &&
       is_delegable_physical_address(old_s, pdev_2_ptr) &&
       GranuleAt(old_s, pdev_2_ptr).state == PDEV &&
       !(PDEVAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE) &&
       !(PDEVAt(old_s, pdev_2_ptr).p2p_stream != stream_ptr) &&
       is_aligned_to_granule_size(old_s, vdev_1_ptr) &&
       is_delegable_physical_address(old_s, vdev_1_ptr) &&
       GranuleAt(old_s, vdev_1_ptr).state == VDEV &&
       !(VDEVAt(old_s, vdev_1_ptr).realm != rd) &&
       !(VDEVAt(old_s, vdev_1_ptr).pdev != pdev_1_ptr) &&
       !(VDEVAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VDEVAt(old_s, vdev_1_ptr).vdev_attest_info_1 != VDEVAt(old_s, rec_ptr).vdev_attest_info_1) &&
       !(VDEVAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE) &&
       is_aligned_to_granule_size(old_s, vdev_2_ptr) &&
       is_delegable_physical_address(old_s, vdev_2_ptr) &&
       GranuleAt(old_s, vdev_2_ptr).state == VDEV &&
       !(VDEVAt(old_s, vdev_2_ptr).realm != rd) &&
       !(VDEVAt(old_s, vdev_2_ptr).pdev != pdev_2_ptr) &&
       !(VDEVAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VDEVAt(old_s, vdev_2_ptr).vdev_attest_info_2 != VDEVAt(old_s, rec_ptr).vdev_attest_info_2) &&
       !(VDEVAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_1_ptr).operation == VDEVAt(old_s, vdev_1_ptr).operation)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_1_ptr).comm_state == VDEVAt(old_s, vdev_1_ptr).comm_state)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_1_ptr).p2p_bound == VDEVAt(old_s, vdev_1_ptr).p2p_bound)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_1_ptr).p2p_stream == VDEVAt(old_s, vdev_1_ptr).p2p_stream)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_1_ptr).p2p_peer == VDEVAt(old_s, vdev_1_ptr).p2p_peer)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_2_ptr).operation == VDEVAt(old_s, vdev_2_ptr).operation)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_2_ptr).comm_state == VDEVAt(old_s, vdev_2_ptr).comm_state)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_2_ptr).p2p_bound == VDEVAt(old_s, vdev_2_ptr).p2p_bound)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_2_ptr).p2p_stream == VDEVAt(old_s, vdev_2_ptr).p2p_stream)
  && (result.is_Err()
    ==> VDEVAt(new_s, vdev_2_ptr).p2p_peer == VDEVAt(old_s, vdev_2_ptr).p2p_peer)
}
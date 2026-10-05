pub open spec fn rmi_vdev_p2p_bind_spec(stream_ptr: Address, rd: Address, rec_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, vdev_1_ptr: Address, vdev_2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (is_aligned_to(old_s, rd, granule_size()) ==> result.is_Ok())
  && (!is_aligned_to(old_s, rd, granule_size()) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, rd) ==> result.is_Ok())
  && (!is_delegable_physical_address(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && (!GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_aligned_to(old_s, rec_ptr, granule_size()) ==> result.is_Ok())
  && (!is_aligned_to(old_s, rec_ptr, granule_size()) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, rec_ptr) ==> result.is_Ok())
  && (!is_delegable_physical_address(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state == REC ==> result.is_Ok())
  && (!GranuleAt(old_s, rec_ptr).state == REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (!RecAt(old_s, rec_ptr).state == REC_RUNNING ==> result.is_Ok())
  && (RecAt(old_s, rec_ptr).realm != RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REC))
  && (!RecAt(old_s, rec_ptr).realm != RealmAt(old_s, rd) ==> result.is_Ok())
  && (is_aligned_to(old_s, stream_ptr, granule_size()) ==> result.is_Ok())
  && (!is_aligned_to(old_s, stream_ptr, granule_size()) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, stream_ptr) ==> result.is_Ok())
  && (!is_delegable_physical_address(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state == P2P_STREAM ==> result.is_Ok())
  && (!GranuleAt(old_s, stream_ptr).state == P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_aligned_to(old_s, pdev_1_ptr, granule_size()) ==> result.is_Ok())
  && (!is_aligned_to(old_s, pdev_1_ptr, granule_size()) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, pdev_1_ptr) ==> result.is_Ok())
  && (!is_delegable_physical_address(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_1_ptr).state == PDEV ==> result.is_Ok())
  && (!GranuleAt(old_s, pdev_1_ptr).state == PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_1_ptr).p2p_stream != P2PStreamAt(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE && PdevAt(old_s, pdev_1_ptr).p2p_stream == P2PStreamAt(old_s, stream_ptr) ==> result.is_Ok())
  && (is_aligned_to(old_s, pdev_2_ptr, granule_size()) ==> result.is_Ok())
  && (!is_aligned_to(old_s, pdev_2_ptr, granule_size()) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, pdev_2_ptr) ==> result.is_Ok())
  && (!is_delegable_physical_address(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_2_ptr).state == PDEV ==> result.is_Ok())
  && (!GranuleAt(old_s, pdev_2_ptr).state == PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_2_ptr).p2p_stream != P2PStreamAt(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE && PdevAt(old_s, pdev_2_ptr).p2p_stream == P2PStreamAt(old_s, stream_ptr) ==> result.is_Ok())
  && (is_aligned_to(old_s, vdev_1_ptr, granule_size()) ==> result.is_Ok())
  && (!is_aligned_to(old_s, vdev_1_ptr, granule_size()) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, vdev_1_ptr) ==> result.is_Ok())
  && (!is_delegable_physical_address(old_s, vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_1_ptr).state == VDEV ==> result.is_Ok())
  && (!GranuleAt(old_s, vdev_1_ptr).state == VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_1_ptr).realm != RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VdevAt(old_s, vdev_1_ptr).realm != RealmAt(old_s, rd) ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).pdev != PdevAt(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VdevAt(old_s, vdev_1_ptr).pdev != PdevAt(old_s, pdev_1_ptr) ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).vdev_attest_info_1 != VdevAt(old_s, vdev_1_ptr).vdev_attest_info_1 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!VdevAt(old_s, vdev_1_ptr).vdev_attest_info_1 != VdevAt(old_s, vdev_1_ptr).vdev_attest_info_1 ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE ==> result.is_Ok())
  && (is_aligned_to(old_s, vdev_2_ptr, granule_size()) ==> result.is_Ok())
  && (!is_aligned_to(old_s, vdev_2_ptr, granule_size()) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, vdev_2_ptr) ==> result.is_Ok())
  && (!is_delegable_physical_address(old_s, vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_2_ptr).state == VDEV ==> result.is_Ok())
  && (!GranuleAt(old_s, vdev_2_ptr).state == VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_2_ptr).realm != RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VdevAt(old_s, vdev_2_ptr).realm != RealmAt(old_s, rd) ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).pdev != PdevAt(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VdevAt(old_s, vdev_2_ptr).pdev != PdevAt(old_s, pdev_2_ptr) ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).vdev_attest_info_2 != VdevAt(old_s, vdev_2_ptr).vdev_attest_info_2 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!VdevAt(old_s, vdev_2_ptr).vdev_attest_info_2 != VdevAt(old_s, vdev_2_ptr).vdev_attest_info_2 ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE ==> result.is_Ok())
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).op == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).p2p_bound == FEATURE_TRUE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).p2p_stream == P2PStreamAt(new_s, stream_ptr))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).p2p_peer == Vdev_id_of(new_s, VdevAt(new_s, vdev_2_ptr)))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).op == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).p2p_bound == FEATURE_TRUE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).p2p_stream == P2PStreamAt(new_s, stream_ptr))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).p2p_peer == Vdev_id_of(new_s, VdevAt(new_s, vdev_1_ptr)))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       is_aligned_to(old_s, rd, granule_size()) &&
       is_delegable_physical_address(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       is_aligned_to(old_s, rec_ptr, granule_size()) &&
       is_delegable_physical_address(old_s, rec_ptr) &&
       GranuleAt(old_s, rec_ptr).state == REC &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       RecAt(old_s, rec_ptr).realm == RealmAt(old_s, rd) &&
       is_aligned_to(old_s, stream_ptr, granule_size()) &&
       is_delegable_physical_address(old_s, stream_ptr) &&
       GranuleAt(old_s, stream_ptr).state == P2P_STREAM &&
       is_aligned_to(old_s, pdev_1_ptr, granule_size()) &&
       is_delegable_physical_address(old_s, pdev_1_ptr) &&
       GranuleAt(old_s, pdev_1_ptr).state == PDEV &&
       !(PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_1_ptr).p2p_stream != P2PStreamAt(old_s, stream_ptr)) &&
       is_aligned_to(old_s, pdev_2_ptr, granule_size()) &&
       is_delegable_physical_address(old_s, pdev_2_ptr) &&
       GranuleAt(old_s, pdev_2_ptr).state == PDEV &&
       !(PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_2_ptr).p2p_stream != P2PStreamAt(old_s, stream_ptr)) &&
       is_aligned_to(old_s, vdev_1_ptr, granule_size()) &&
       is_delegable_physical_address(old_s, vdev_1_ptr) &&
       GranuleAt(old_s, vdev_1_ptr).state == VDEV &&
       VdevAt(old_s, vdev_1_ptr).realm == RealmAt(old_s, rd) &&
       VdevAt(old_s, vdev_1_ptr).pdev == PdevAt(old_s, pdev_1_ptr) &&
       !(VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VdevAt(old_s, vdev_1_ptr).vdev_attest_info_1 != VdevAt(old_s, vdev_1_ptr).vdev_attest_info_1) &&
       !(VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE) &&
       is_aligned_to(old_s, vdev_2_ptr, granule_size()) &&
       is_delegable_physical_address(old_s, vdev_2_ptr) &&
       GranuleAt(old_s, vdev_2_ptr).state == VDEV &&
       VdevAt(old_s, vdev_2_ptr).realm == RealmAt(old_s, rd) &&
       VdevAt(old_s, vdev_2_ptr).pdev == PdevAt(old_s, pdev_2_ptr) &&
       !(VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VdevAt(old_s, vdev_2_ptr).vdev_attest_info_2 != VdevAt(old_s, vdev_2_ptr).vdev_attest_info_2) &&
       !(VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).op == VdevAt(old_s, vdev_1_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).comm_state == VdevAt(old_s, vdev_1_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).p2p_bound == VdevAt(old_s, vdev_1_ptr).p2p_bound)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).p2p_stream == VdevAt(old_s, vdev_1_ptr).p2p_stream)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).p2p_peer == VdevAt(old_s, vdev_1_ptr).p2p_peer)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).op == VdevAt(old_s, vdev_2_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).comm_state == VdevAt(old_s, vdev_2_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).p2p_bound == VdevAt(old_s, vdev_2_ptr).p2p_bound)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).p2p_stream == VdevAt(old_s, vdev_2_ptr).p2p_stream)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).p2p_peer == VdevAt(old_s, vdev_2_ptr).p2p_peer)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_1_ptr).op != VDEV_OP_P2P_BIND))
    ==> VdevAt(new_s, vdev_1_ptr).op == VdevAt(old_s, vdev_1_ptr).op)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_1_ptr).comm_state != DEV_COMM_PENDING))
    ==> VdevAt(new_s, vdev_1_ptr).comm_state == VdevAt(old_s, vdev_1_ptr).comm_state)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_1_ptr).p2p_bound != FEATURE_TRUE))
    ==> VdevAt(new_s, vdev_1_ptr).p2p_bound == VdevAt(old_s, vdev_1_ptr).p2p_bound)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_1_ptr).p2p_stream != P2PStreamAt(new_s, stream_ptr)))
    ==> VdevAt(new_s, vdev_1_ptr).p2p_stream == VdevAt(old_s, vdev_1_ptr).p2p_stream)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_1_ptr).p2p_peer != Vdev_id_of(new_s, VdevAt(new_s, vdev_2_ptr))))
    ==> VdevAt(new_s, vdev_1_ptr).p2p_peer == VdevAt(old_s, vdev_1_ptr).p2p_peer)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_2_ptr).op != VDEV_OP_P2P_BIND))
    ==> VdevAt(new_s, vdev_2_ptr).op == VdevAt(old_s, vdev_2_ptr).op)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_2_ptr).comm_state != DEV_COMM_PENDING))
    ==> VdevAt(new_s, vdev_2_ptr).comm_state == VdevAt(old_s, vdev_2_ptr).comm_state)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_2_ptr).p2p_bound != FEATURE_TRUE))
    ==> VdevAt(new_s, vdev_2_ptr).p2p_bound == VdevAt(old_s, vdev_2_ptr).p2p_bound)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_2_ptr).p2p_stream != P2PStreamAt(new_s, stream_ptr)))
    ==> VdevAt(new_s, vdev_2_ptr).p2p_stream == VdevAt(old_s, vdev_2_ptr).p2p_stream)
  && (!(result.is_Ok() &&
       (VdevAt(new_s, vdev_2_ptr).p2p_peer != Vdev_id_of(new_s, VdevAt(new_s, vdev_1_ptr))))
    ==> VdevAt(new_s, vdev_2_ptr).p2p_peer == VdevAt(old_s, vdev_2_ptr).p2p_peer)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == PdevAt(old_s, pdev_1_ptr).p2p_stream_valid)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).p2p_stream == PdevAt(old_s, pdev_1_ptr).p2p_stream)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == PdevAt(old_s, pdev_2_ptr).p2p_stream_valid)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).p2p_stream == PdevAt(old_s, pdev_2_ptr).p2p_stream)
}
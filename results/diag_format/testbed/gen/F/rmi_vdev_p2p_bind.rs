pub open spec fn rmi_vdev_p2p_bind_spec(stream_ptr: Address, rd: Address, rec_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, vdev_1_ptr: Address, vdev_2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!crate::RmmFeature::Equal(old_s, crate::RmmFeature::FEATURE_TRUE) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_NOT_SUPPORTED))
    && (!crate::AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, rd) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::GranuleAt(old_s, rd).state == crate::RmmGranuleState::RD ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::GranuleAt(old_s, rec_ptr).state == crate::RmmGranuleState::REC ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (crate::RecAt(old_s, rec_ptr).state == crate::RmmRecState::REC_RUNNING ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_REC))
    && (!crate::RecAt(old_s, rec_ptr).owner == crate::RealmAt(old_s, rd).meCID ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_REC))
    && (!crate::AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::GranuleAt(old_s, stream_ptr).state == crate::RmmGranuleState::P2P_STREAM ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, pdev_1_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::GranuleAt(old_s, pdev_1_ptr).state == crate::RmmGranuleState::PDEV ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PdevAt(old_s, pdev_1_ptr).p2p_stream_valid == crate::RmmBoolean::RMM_TRUE ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PdevAt(old_s, pdev_1_ptr).p2p_stream == stream_ptr ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, pdev_2_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::GranuleAt(old_s, pdev_2_ptr).state == crate::RmmGranuleState::PDEV ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PdevAt(old_s, pdev_2_ptr).p2p_stream_valid == crate::RmmBoolean::RMM_TRUE ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PdevAt(old_s, pdev_2_ptr).p2p_stream == stream_ptr ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::AddrIsGranuleAligned(old_s, vdev_1_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, vdev_1_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::GranuleAt(old_s, vdev_1_ptr).state == crate::RmmGranuleState::VDEV ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::VdevAt(old_s, vdev_1_ptr).realm == crate::RealmAt(old_s, rd).meCID ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::VdevAt(old_s, vdev_1_ptr).pdev == pdev_1_ptr ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::VdevAt(old_s, vdev_1_ptr).comm_state == crate::RmmDevCommState::DEV_COMM_IDLE ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_DEVICE))
    && (!crate::VdevAttestInfoEqual1(crate::VdevAt(old_s, vdev_1_ptr).attest_info, crate::RecAt(old_s, rec_ptr).vdev_attest_info_1) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_DEVICE))
    && (!crate::RmmFeature::Equal(crate::VdevAt(old_s, vdev_1_ptr).p2p_bound, crate::RmmFeature::RMM_FALSE) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_DEVICE))
    && (!crate::AddrIsGranuleAligned(old_s, vdev_2_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::PaIsDelegable(old_s, vdev_2_ptr) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::GranuleAt(old_s, vdev_2_ptr).state == crate::RmmGranuleState::VDEV ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::VdevAt(old_s, vdev_2_ptr).realm == crate::RealmAt(old_s, rd).meCID ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::VdevAt(old_s, vdev_2_ptr).pdev == pdev_2_ptr ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_INPUT))
    && (!crate::VdevAt(old_s, vdev_2_ptr).comm_state == crate::RmmDevCommState::DEV_COMM_IDLE ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_DEVICE))
    && (!crate::VdevAttestInfoEqual1(crate::VdevAt(old_s, vdev_2_ptr).attest_info, crate::RecAt(old_s, rec_ptr).vdev_attest_info_2) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_DEVICE))
    && (!crate::RmmFeature::Equal(crate::VdevAt(old_s, vdev_2_ptr).p2p_bound, crate::RmmFeature::RMM_FALSE) ==> ResultEqual(result, crate::RmiStatusCode::RMI_ERROR_DEVICE))
    && (result.is_Ok() ==> (
        new_s.VdevAt(vdev_1_ptr).op == crate::RmmVdevOperation::VDEV_OP_P2P_BIND
        && new_s.VdevAt(vdev_1_ptr).comm_state == crate::RmmDevCommState::DEV_COMM_PENDING
        && new_s.VdevAt(vdev_1_ptr).p2p_bound == crate::RmmFeature::RMM_TRUE
        && new_s.VdevAt(vdev_1_ptr).p2p_stream == stream_ptr
        && new_s.VdevAt(vdev_1_ptr).p2p_peer == new_s.VdevAt(vdev_2_ptr).vdev_id
        && new_s.VdevAt(vdev_2_ptr).op == crate::RmmVdevOperation::VDEV_OP_P2P_BIND
        && new_s.VdevAt(vdev_2_ptr).comm_state == crate::RmmDevCommState::DEV_COMM_PENDING
        && new_s.VdevAt(vdev_2_ptr).p2p_bound == crate::RmmFeature::RMM_TRUE
        && new_s.VdevAt(vdev_2_ptr).p2p_stream == stream_ptr
        && new_s.VdevAt(vdev_2_ptr).p2p_peer == new_s.VdevAt(vdev_1_ptr).vdev_id
    ))
}
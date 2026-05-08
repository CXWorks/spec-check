```verus
pub open spec fn RMI_PDEV_P2P_DISCONNECT_spec(s: S, stream_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address) -> bool {
  let stream = P2PStreamAt(s, stream_ptr);
  let pdev_1 = PdevAt(s, pdev_1_ptr);
  let pdev_2 = PdevAt(s, pdev_2_ptr);
  
  (
    // da_supp failure condition
    (!ImplFeatures(s).feat_da.is_FEATURE_TRUE() ==> ResultEqual(RMI_ERROR_NOT_SUPPORTED, RMI_ERROR_NOT_SUPPORTED))
    &&
    // stream_align failure condition
    (!AddrIsGranuleAligned(s, stream_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // stream_bound failure condition
    (!PaIsDelegable(s, stream_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // stream_state failure condition
    (GranuleAt(s, stream_ptr).state != RmmGranuleState::P2P_STREAM ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // pdev_1_align failure condition
    (!AddrIsGranuleAligned(s, pdev_1_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // pdev_1_bound failure condition
    (!PaIsDelegable(s, pdev_1_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // pdev_1_gran_state failure condition
    (GranuleAt(s, pdev_1_ptr).state != RmmGranuleState::PDEV ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // pdev_2_align failure condition
    (!AddrIsGranuleAligned(s, pdev_2_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // pdev_2_bound failure condition
    (!PaIsDelegable(s, pdev_2_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // pdev_2_gran_state failure condition
    (GranuleAt(s, pdev_2_ptr).state != RmmGranuleState::PDEV ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // stream validity condition
    ((pdev_1.p2p_stream_valid != RmmBool::RMM_TRUE || pdev_1.p2p_stream != stream_ptr || pdev_2.p2p_stream_valid != RmmBool::RMM_TRUE || pdev_2.p2p_stream != stream_ptr) ==> ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
    &&
    // Success conditions when all preconditions pass
    (
      (ImplFeatures(s).feat_da.is_FEATURE_TRUE() && AddrIsGranuleAligned(s, stream_ptr) && PaIsDelegable(s, stream_ptr) && GranuleAt(s, stream_ptr).state == RmmGranuleState::P2P_STREAM && AddrIsGranuleAligned(s, pdev_1_ptr) && PaIsDelegable(s, pdev_1_ptr) && GranuleAt(s, pdev_1_ptr).state == RmmGranuleState::PDEV && AddrIsGranuleAligned(s, pdev_2_ptr) && PaIsDelegable(s, pdev_2_ptr) && GranuleAt(s, pdev_2_ptr).state == RmmGranuleState::PDEV && pdev_1.p2p_stream_valid == RmmBool::RMM_TRUE && pdev_1.p2p_stream == stream_ptr && pdev_2.p2p_stream_valid == RmmBool::RMM_TRUE && pdev_2.p2p_stream == stream_ptr)
      ==>
      (
        GranuleAt(s, stream_ptr).state == RmmGranuleState::DELEGATED
        && pdev_1.p2p_stream_valid == RmmBool::RMM_FALSE
        && pdev_1.state == RmmPdevState::PDEV_COMMUNICATING
        && pdev_1.comm_state == RmmDevCommState::DEV_COMM_PENDING
        && pdev_2.p2p_stream_valid == RmmBool::RMM_FALSE
        && pdev_2.state == RmmPdevState::PDEV_COMMUNICATING
        && pdev_2.comm_state == RmmDevCommState::DEV_COMM_PENDING
      )
    )
  )
}
```
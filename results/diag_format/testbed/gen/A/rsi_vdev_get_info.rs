pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (CurrentRealm(old_s).feat_da == FEATURE_FALSE
        ==> (result == RSI_ERROR_STATE
            || (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas == EMPTY
                && result == RSI_ERROR_INPUT)))
    && ((CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id))
        ==> result == RSI_ERROR_INPUT)
    && ((CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && !AddrIsAligned(old_s, addr, 512))
        ==> result == RSI_ERROR_INPUT)
    && ((CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && !AddrIsProtected(old_s, addr, CurrentRealm(old_s)))
        ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas == EMPTY
        ==> (result == RSI_ERROR_INPUT
            || (CurrentRealm(old_s).feat_da == FEATURE_FALSE && result == RSI_ERROR_STATE)))
    && ((CurrentRealm(old_s).feat_da == FEATURE_TRUE
        && !VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)
        && AddrIsAligned(old_s, addr, 512)
        && AddrIsProtected(old_s, addr, CurrentRealm(old_s))
        && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != EMPTY)
        ==> (result == RSI_SUCCESS
            && (PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).pdev).hash_algo == HASH_SHA_256
                ==> RsiVdevInfoAt(new_s, addr).hash_algo == RSI_HASH_SHA_256)
            && (PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).pdev).hash_algo == HASH_SHA_512
                ==> RsiVdevInfoAt(new_s, addr).hash_algo == RSI_HASH_SHA_512)
            && RsiVdevInfoAt(new_s, addr).flags.p2p_enabled
                == FeatureToRsi(new_s, PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).pdev).p2p_enabled)
            && RsiVdevInfoAt(new_s, addr).flags.p2p_bound
                == FeatureToRsi(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).p2p_bound)
            && RsiVdevInfoAt(new_s, addr).p2p_peer
                == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).p2p_peer
            && VdevAttestInfoEqual(
                RsiVdevInfoAt(new_s, addr).lock_nonce as int,
                RsiVdevInfoAt(new_s, addr).meas_nonce as int,
                RsiVdevInfoAt(new_s, addr).report_nonce as int,
                VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).attest_info)
            && RsiVdevInfoAt(new_s, addr).vca_digest
                == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).pdev).vca_digest
            && RsiVdevInfoAt(new_s, addr).meas_digest
                == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).meas_digest
            && RsiVdevInfoAt(new_s, addr).report_digest
                == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).report_digest
            && (VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).vdev_state == VDEV_NEW
                ==> RsiVdevInfoAt(new_s, addr).state == RSI_VDEV_NEW)
            && (VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).vdev_state == VDEV_UNLOCKED
                ==> RsiVdevInfoAt(new_s, addr).state == RSI_VDEV_UNLOCKED)
            && (VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).vdev_state == VDEV_LOCKED
                ==> RsiVdevInfoAt(new_s, addr).state == RSI_VDEV_LOCKED)
            && (VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).vdev_state == VDEV_STARTED
                ==> RsiVdevInfoAt(new_s, addr).state == RSI_VDEV_STARTED)
            && (VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).vdev_state == VDEV_ERROR
                ==> RsiVdevInfoAt(new_s, addr).state == RSI_VDEV_ERROR)))
}

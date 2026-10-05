pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).device_assignment_feature == false ==> result == RSI_ERROR_STATE)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned == false ==> result == RSI_ERROR_INPUT)
  && ((addr) % 512 != 0 ==> result == RSI_ERROR_INPUT)
  && (!is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY ==> result == RSI_ERROR_INPUT)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> result == RSI_SUCCESS)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).hash_algorithm == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).hash_algorithm)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).p2p_enabled == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).p2p_enabled)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).p2p_bound == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).p2p_bound)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).p2p_peer == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).p2p_peer)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).lock_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).lock_nonce)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).measurement_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).measurement_nonce)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).report_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).report_nonce)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).vca_digest == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).vca_digest)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).measurement_digest == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).measurement_digest)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).report_digest == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).report_digest)
  && (result == RSI_ERROR_INPUT || result == RSI_ERROR_STATE || result == RSI_ERROR_UNKNOWN ==> RsiVdevInfoAt(new_s, addr).state == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).state)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).hash_algorithm == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).hash_algorithm)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).p2p_enabled == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).p2p_enabled)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).p2p_bound == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).p2p_bound)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).p2p_peer == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).p2p_peer)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).lock_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).lock_nonce)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).measurement_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).measurement_nonce)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).report_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).report_nonce)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).vca_digest == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).vca_digest)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).measurement_digest == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).measurement_digest)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).report_digest == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).report_digest)
  && ((!(CurrentRealm(old_s).device_assignment_feature == false) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).assigned &&
       ((addr) % 512 == 0) &&
       is_protected_ipa_space(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RIPAS_EMPTY))
    ==> RsiVdevInfoAt(new_s, addr).state == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).state)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).hash_algorithm == RsiVdevInfoAt(old_s, addr).hash_algorithm)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_enabled == RsiVdevInfoAt(old_s, addr).p2p_enabled)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_bound == RsiVdevInfoAt(old_s, addr).p2p_bound)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_peer == RsiVdevInfoAt(old_s, addr).p2p_peer)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).lock_nonce == RsiVdevInfoAt(old_s, addr).lock_nonce)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).measurement_nonce == RsiVdevInfoAt(old_s, addr).measurement_nonce)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_nonce == RsiVdevInfoAt(old_s, addr).report_nonce)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).vca_digest == RsiVdevInfoAt(old_s, addr).vca_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).measurement_digest == RsiVdevInfoAt(old_s, addr).measurement_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_digest == RsiVdevInfoAt(old_s, addr).report_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).state == RsiVdevInfoAt(old_s, addr).state)
}
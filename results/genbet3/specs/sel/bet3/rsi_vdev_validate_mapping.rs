pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: Bits64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_seq: UInt64, response: RsiResponse, new_ipa_base: Address, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && ((VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).vdev_state != VDEV_STARTED) ==> result == RSI_ERROR_DEVICE)
  && (!AddrIsRsiGranuleAligned(old_s, ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsRsiGranuleAligned(old_s, ipa_top) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsRsiGranuleAligned(old_s, pa_base) ==> result == RSI_ERROR_INPUT)
  && ((ipa_top) <= (ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && flags.coh == RSI_DEV_MEM_NON_COHERENT ==> CurrentRec(new_s).dev_mem_flags.coh == DEV_MEM_NON_COHERENT)
  && (result == RSI_SUCCESS && flags.coh == RSI_DEV_MEM_COHERENT ==> CurrentRec(new_s).dev_mem_flags.coh == DEV_MEM_COHERENT)
  && (result == RSI_SUCCESS && flags.order == RSI_DEV_MEM_NOT_LIMITED_ORDER ==> CurrentRec(new_s).dev_mem_flags.order == DEV_MEM_NOT_LIMITED_ORDER)
  && (result == RSI_SUCCESS && flags.order == RSI_DEV_MEM_LIMITED_ORDER ==> CurrentRec(new_s).dev_mem_flags.order == DEV_MEM_LIMITED_ORDER)
  && (result == RSI_SUCCESS ==> new_ipa_base == CurrentRec(new_s).dev_mem_addr)
  && (result == RSI_SUCCESS ==> response == RecDevMemResponseToRsi(new_s, CurrentRec(new_s)))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       !(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)) &&
       !((VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).vdev_state != VDEV_LOCKED && VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).vdev_state != VDEV_STARTED)) &&
       AddrIsRsiGranuleAligned(old_s, ipa_base) &&
       AddrIsRsiGranuleAligned(old_s, ipa_top) &&
       AddrIsRsiGranuleAligned(old_s, pa_base) &&
       !((ipa_top) <= (ipa_base)) &&
       AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRec(new_s).dev_mem_flags.coh == CurrentRec(old_s).dev_mem_flags.coh)
  && (result != RSI_SUCCESS
    ==> CurrentRec(new_s).dev_mem_flags.order == CurrentRec(old_s).dev_mem_flags.order)
  && (result != RSI_SUCCESS
    ==> CurrentRec(new_s).dev_mem_addr == CurrentRec(old_s).dev_mem_addr)
}
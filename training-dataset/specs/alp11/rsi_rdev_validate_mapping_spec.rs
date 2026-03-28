pub open spec fn rsi_rdev_validate_mapping_spec(vdev_id: Bits64, inst_id: UInt64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, ipa_top) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, pa_base) ==> result == RSI_ERROR_INPUT)
  && ((ipa_top) <= (ipa_base) ==> result == RSI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> new_ipa_base == CurrentRec(new_s).dev_mem_addr)
  && (result == RSI_SUCCESS ==> response == RecDevMemResponseToRsi(new_s, CurrentRec(new_s)))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED)) &&
       AddrIsGranuleAligned(old_s, ipa_base) &&
       AddrIsGranuleAligned(old_s, ipa_top) &&
       AddrIsGranuleAligned(old_s, pa_base) &&
       !((ipa_top) <= (ipa_base)) &&
       AddrRangeIsProtected(old_s, ipa_base, ipa_top, CurrentRealm(old_s)))
    ==> result == RSI_SUCCESS)
}

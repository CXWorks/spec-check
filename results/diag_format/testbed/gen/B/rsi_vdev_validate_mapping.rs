pub open spec fn rsi_vdev_validate_mapping_spec(vdev_id: Bits64, ipa_base: Address, ipa_top: Address, pa_base: Address, flags: RsiDevMemFlags, lock_nonce: UInt64, meas_nonce: UInt64, report_nonce: UInt64, result: RsiCommandReturnCode, new_ipa_base: Address, response: RsiResponse, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    (realm.feat_da == RmmFeature::FEATURE_FALSE ==> result == RsiCommandReturnCode::RSI_ERROR_STATE)
    && (VdevIdIsFree(old_s, realm, vdev_id) ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
    && (vdev.vdev_state != RmmVdevState::VDEV_LOCKED && vdev.vdev_state != RmmVdevState::VDEV_STARTED ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, ipa_base) ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, ipa_top) ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, pa_base) ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
    && (ipa_top <= ipa_base ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, realm) ==> result == RsiCommandReturnCode::RSI_ERROR_INPUT)
    && (!VdevAttestInfoEqual(lock_nonce as int, meas_nonce as int, report_nonce as int, vdev.attest_info) ==> result == RsiCommandReturnCode::RSI_ERROR_DEVICE)
    && (result == RsiCommandReturnCode::RSI_SUCCESS ==> new_ipa_base == rec.dev_mem_addr)
    && (result == RsiCommandReturnCode::RSI_SUCCESS ==> response == RecDevMemResponseToRsi(old_s, rec))
}
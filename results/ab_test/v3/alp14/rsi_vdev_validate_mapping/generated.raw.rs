pub open spec fn rsi_vdev_validate_mapping_spec(
    result: RsiCommandReturnCode,
    vdev_id: u64,
    ipa_base: Address,
    ipa_top: Address,
    pa_base: Address,
    flags: RsiDevMemFlags,
    lock_nonce: u64,
    meas_nonce: u64,
    report_nonce: u64,
    new_ipa_base: Address,
    response: RsiResponse,
    old_s: S,
    new_s: S
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    
    (realm.feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
    && (VdevIdIsFree(old_s, realm, vdev_id) ==> result == RSI_ERROR_INPUT)
    && ((vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(ipa_base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(ipa_top) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(pa_base) ==> result == RSI_ERROR_INPUT)
    && (((ipa_top as int) <= (ipa_base as int)) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(ipa_base, ipa_top, realm) ==> result == RSI_ERROR_INPUT)
    && (!VdevAttestInfoEqual(lock_nonce as int, meas_nonce as int, report_nonce as int, vdev.attest_info) ==> result == RSI_ERROR_DEVICE)
    && (
        (realm.feat_da == FEATURE_TRUE
        && !VdevIdIsFree(old_s, realm, vdev_id)
        && (vdev.vdev_state == VDEV_LOCKED || vdev.vdev_state == VDEV_STARTED)
        && AddrIsGranuleAligned(ipa_base)
        && AddrIsGranuleAligned(ipa_top)
        && AddrIsGranuleAligned(pa_base)
        && ((ipa_top as int) > (ipa_base as int))
        && AddrRangeIsProtected(ipa_base, ipa_top, realm)
        && VdevAttestInfoEqual(lock_nonce as int, meas_nonce as int, report_nonce as int, vdev.attest_info))
        ==> (result.is_Ok()
            && new_ipa_base == rec.dev_mem_addr
            && response == RecDevMemResponseToRsi(old_s, rec))
    )
}
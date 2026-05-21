pub open spec fn rsi_vdev_validate_mapping_spec(
    result: RsiCommandReturnCode,
    new_ipa_base: Address,
    response: RsiResponse,
    old_s: S,
    new_s: S,
    vdev_id: u64,
    ipa_base: Address,
    ipa_top: Address,
    pa_base: Address,
    flags: RsiDevMemFlags,
    lock_nonce: int,
    meas_nonce: int,
    report_nonce: int
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let vdev = VdevFromVdevId(old_s, realm, vdev_id);
    
    (!realm.feat_da_eq_true(old_s) ==> result == RSI_ERROR_STATE)
    && (VdevIdIsFree(old_s, realm, vdev_id) ==> result == RSI_ERROR_INPUT)
    && ((vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, ipa_base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, ipa_top) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, pa_base) ==> result == RSI_ERROR_INPUT)
    && (UInt(ipa_top) <= UInt(ipa_base) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, ipa_base, ipa_top, realm) ==> result == RSI_ERROR_INPUT)
    && (!VdevAttestInfoEqual(lock_nonce, meas_nonce, report_nonce, vdev.attest_info) ==> result == RSI_ERROR_DEVICE)
    && ((realm.feat_da_eq_true(old_s)
        && !VdevIdIsFree(old_s, realm, vdev_id)
        && (vdev.vdev_state == VDEV_LOCKED || vdev.vdev_state == VDEV_STARTED)
        && AddrIsGranuleAligned(old_s, ipa_base)
        && AddrIsGranuleAligned(old_s, ipa_top)
        && AddrIsGranuleAligned(old_s, pa_base)
        && UInt(ipa_top) > UInt(ipa_base)
        && AddrRangeIsProtected(old_s, ipa_base, ipa_top, realm)
        && VdevAttestInfoEqual(lock_nonce, meas_nonce, report_nonce, vdev.attest_info))
        ==> (result == RSI_SUCCESS
            && new_ipa_base == rec.dev_mem_addr
            && response == RecDevMemResponseToRsi(old_s, rec)
            && new_s == old_s))
}
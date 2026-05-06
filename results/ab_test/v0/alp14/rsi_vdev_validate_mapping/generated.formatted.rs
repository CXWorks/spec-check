pub open spec fn RSI_VDEV_VALIDATE_MAPPING_spec(
    s: S,
    vdev_id: u64,
    ipa_base: Address,
    ipa_top: Address,
    pa_base: Address,
    flags: RsiDevMemFlags,
    lock_nonce: int,
    meas_nonce: int,
    report_nonce: int,
    result: RsiCommandReturnCode,
    new_ipa_base: Address,
    response: RsiResponse,
) -> bool {
    let realm = CurrentRealm(s);
    let rec = CurrentRec(s);
    let vdev = VdevFromVdevId(s, realm, vdev_id);

    let da_en_fail = realm.feat_da != FEATURE_TRUE && result == RSI_ERROR_STATE;
    let vdev_id_fail = VdevIdIsFree(s, realm, vdev_id) && result == RSI_ERROR_INPUT;
    let state_fail = (vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED) && result
        == RSI_ERROR_INPUT;
    let ipa_base_align_fail = !AddrIsGranuleAligned(ipa_base) && result == RSI_ERROR_INPUT;
    let ipa_top_align_fail = !AddrIsGranuleAligned(ipa_top) && result == RSI_ERROR_INPUT;
    let pa_align_fail = !AddrIsGranuleAligned(pa_base) && result == RSI_ERROR_INPUT;
    let size_valid_fail = UInt(ipa_top) <= UInt(ipa_base) && result == RSI_ERROR_INPUT;
    let rgn_bound_fail = !AddrRangeIsProtected(ipa_base, ipa_top, realm) && result
        == RSI_ERROR_INPUT;
    let attest_info_fail = !VdevAttestInfoEqual(
        lock_nonce,
        meas_nonce,
        report_nonce,
        vdev.attest_info,
    ) && result == RSI_ERROR_DEVICE;

    let success = realm.feat_da == FEATURE_TRUE && !VdevIdIsFree(s, realm, vdev_id) && (
    vdev.vdev_state == VDEV_LOCKED || vdev.vdev_state == VDEV_STARTED) && AddrIsGranuleAligned(
        ipa_base,
    ) && AddrIsGranuleAligned(ipa_top) && AddrIsGranuleAligned(pa_base) && UInt(ipa_top) > UInt(
        ipa_base,
    ) && AddrRangeIsProtected(ipa_base, ipa_top, realm) && VdevAttestInfoEqual(
        lock_nonce,
        meas_nonce,
        report_nonce,
        vdev.attest_info,
    ) && result == RSI_SUCCESS && new_ipa_base == rec.dev_mem_addr && response
        == RecDevMemResponseToRsi(rec);

    success
}
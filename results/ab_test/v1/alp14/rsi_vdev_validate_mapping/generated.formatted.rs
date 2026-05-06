pub open spec fn RSI_VDEV_VALIDATE_MAPPING_spec(
    s: S,
    vdev_id: u64,
    ipa_base: Address,
    ipa_top: Address,
    pa_base: Address,
    flags: RsiDevMemFlags,
    lock_nonce: u64,
    meas_nonce: u64,
    report_nonce: u64,
) -> (RsiCommandReturnCode, Address, RsiResponse) {
    let realm = CurrentRealm();
    let rec = CurrentRec();
    let vdev = VdevFromVdevId(s, realm, vdev_id);

    // Failure condition: da_en
    if realm.feat_da != FEATURE_TRUE {
        return (RSI_ERROR_STATE, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: vdev_id

    if VdevIdIsFree(s, realm, vdev_id) {
        return (RSI_ERROR_INPUT, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: state

    if vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED {
        return (RSI_ERROR_INPUT, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: ipa_base_align

    if !AddrIsGranuleAligned(ipa_base) {
        return (RSI_ERROR_INPUT, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: ipa_top_align

    if !AddrIsGranuleAligned(ipa_top) {
        return (RSI_ERROR_INPUT, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: pa_align

    if !AddrIsGranuleAligned(pa_base) {
        return (RSI_ERROR_INPUT, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: size_valid

    if UInt(ipa_top) <= UInt(ipa_base) {
        return (RSI_ERROR_INPUT, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: rgn_bound

    if !AddrRangeIsProtected(ipa_base, ipa_top, realm) {
        return (RSI_ERROR_INPUT, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Failure condition: attest_info

    if !VdevAttestInfoEqual(lock_nonce, meas_nonce, report_nonce, vdev.attest_info) {
        return (RSI_ERROR_DEVICE, 0 as Address, VDEV_RESPONSE_REJECT);
    }
    // Success conditions

    let new_ipa_base = rec.dev_mem_addr;
    let response = RecDevMemResponseToRsi(s, rec);

    (RSI_SUCCESS, new_ipa_base, response)
}
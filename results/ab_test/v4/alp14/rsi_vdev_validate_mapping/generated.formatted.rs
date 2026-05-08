pub open spec fn RSI_VDEV_VALIDATE_MAPPING_spec(
    s: S,
    vdev_id: Bits64,
    ipa_base: Address,
    ipa_top: Address,
    pa_base: Address,
    flags: RsiDevMemFlags,
    lock_nonce: u64,
    meas_nonce: u64,
    report_nonce: u64,
    result: RsiCommandReturnCode,
    new_ipa_base: Address,
    response: RsiResponse,
) -> bool {
    let realm = CurrentRealm();
    let rec = CurrentRec();
    let vdev = VdevFromVdevId(realm, vdev_id as int);

    (realm.feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE) && (VdevIdIsFree(
        s,
        realm,
        vdev_id as int,
    ) ==> result == RSI_ERROR_INPUT) && ((vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state
        != VDEV_STARTED) ==> result == RSI_ERROR_INPUT) && (!AddrIsGranuleAligned(s, ipa_base)
        ==> result == RSI_ERROR_INPUT) && (!AddrIsGranuleAligned(s, ipa_top) ==> result
        == RSI_ERROR_INPUT) && (!AddrIsGranuleAligned(s, pa_base) ==> result == RSI_ERROR_INPUT)
        && ((ipa_top as int) <= (ipa_base as int) ==> result == RSI_ERROR_INPUT) && (
    !AddrRangeIsProtected(s, ipa_base, ipa_top, realm) ==> result == RSI_ERROR_INPUT) && (
    !VdevAttestInfoEqual(
        lock_nonce as int,
        meas_nonce as int,
        report_nonce as int,
        vdev.attest_info,
    ) ==> result == RSI_ERROR_DEVICE) && (result == RSI_SUCCESS ==> new_ipa_base
        == rec.dev_mem_addr) && (result == RSI_SUCCESS ==> response == RecDevMemResponseToRsi(
        s,
        rec,
    ))
}
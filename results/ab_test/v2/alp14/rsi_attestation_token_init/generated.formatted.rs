pub open spec fn rsi_attestation_token_init_spec(
    result: RsiCommandReturnCode,
    size: u64,
    old_s: S,
    new_s: S,
    realm: RmmRealm,
    rec: RmmRec,
    challenge_0: u64,
    challenge_1: u64,
    challenge_2: u64,
    challenge_3: u64,
    challenge_4: u64,
    challenge_5: u64,
    challenge_6: u64,
    challenge_7: u64,
) -> bool {
    let challenge = (((challenge_0 as int << 512) | (challenge_1 as int << 448)) | ((
    challenge_2 as int << 384) | (challenge_3 as int << 320))) | (((challenge_4 as int << 256) | (
    challenge_5 as int << 192)) | ((challenge_6 as int << 128) | (challenge_7 as int)));

    result == RSI_SUCCESS && new_s.recs[rec@].attest_state == RmmAttestState::IN_PROGRESS
        && new_s.recs[rec@].attest_challenge == challenge && size == AttestationTokenMaxSize(
        old_s,
        realm,
    )
}
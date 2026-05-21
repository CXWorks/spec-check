pub open spec fn rsi_attestation_token_init_spec(
    result: RsiCommandReturnCode,
    challenge_0: u64,
    challenge_1: u64,
    challenge_2: u64,
    challenge_3: u64,
    challenge_4: u64,
    challenge_5: u64,
    challenge_6: u64,
    challenge_7: u64,
    size: u64,
    old_s: S,
    new_s: S
) -> bool {
    let realm = old_s.CurrentRealm();
    let rec = old_s.CurrentRec();
    let expected_challenge = (((challenge_0 :: challenge_1) :: (challenge_2 :: challenge_3)) :: ((challenge_4 :: challenge_5) :: (challenge_6 :: challenge_7)));
    let expected_size = AttestationTokenMaxSize(realm);
    (result == RSI_OK && new_s.CurrentRec().attest_state == ATTEST_IN_PROGRESS && new_s.CurrentRec().attest_challenge == expected_challenge && size == expected_size)
}
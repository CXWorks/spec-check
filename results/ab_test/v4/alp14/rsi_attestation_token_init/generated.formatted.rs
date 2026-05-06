pub open spec fn RSI_ATTESTATION_TOKEN_INIT_spec(
    old_s: S,
    new_s: S,
    challenge_0: u64,
    challenge_1: u64,
    challenge_2: u64,
    challenge_3: u64,
    challenge_4: u64,
    challenge_5: u64,
    challenge_6: u64,
    challenge_7: u64,
    result: u64,
    size: u64,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let expected_challenge = ((ToBits64(challenge_0 as int) ++ ToBits64(challenge_1 as int)) ++ (ToBits64(challenge_2 as int) ++ ToBits64(challenge_3 as int))) ++ ((ToBits64(challenge_4 as int) ++ ToBits64(challenge_5 as int)) ++ (ToBits64(challenge_6 as int) ++ ToBits64(challenge_7 as int)));
    let expected_size = AttestationTokenMaxSize(old_s, realm);
    (result == RSI_SUCCESS as u64) &&
    (rec.attest_state == RmmAttestState::ATTEST_IN_PROGRESS) &&
    (rec.attest_challenge == expected_challenge) &&
    (size == expected_size)
}
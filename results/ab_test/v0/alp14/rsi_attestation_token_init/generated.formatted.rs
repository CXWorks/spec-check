pub open spec fn RSI_ATTESTATION_TOKEN_INIT_spec(
    s: S,
    challenge_0: u64,
    challenge_1: u64,
    challenge_2: u64,
    challenge_3: u64,
    challenge_4: u64,
    challenge_5: u64,
    challenge_6: u64,
    challenge_7: u64,
    post_rec: RmmRec,
    post_size: u64,
) -> bool {
    let realm = CurrentRealm(s);
    let rec = CurrentRec(s);
    let challenge = concat64(
        concat64(concat64(challenge_0, challenge_1), concat64(challenge_2, challenge_3)),
        concat64(concat64(challenge_4, challenge_5), concat64(challenge_6, challenge_7)),
    );

    post_rec.attest_state == RmmAttestState::ATTEST_IN_PROGRESS && post_rec.attest_challenge
        == challenge && post_size == AttestationTokenMaxSize(s, realm)
}
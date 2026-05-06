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
    result: RsiCommandReturnCode,
    size: u64,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    let expected_challenge = ConcatenateBytes64(
        ConcatenateBytes64(
            ConcatenateBytes64(challenge_0, challenge_1),
            ConcatenateBytes64(challenge_2, challenge_3)
        ),
        ConcatenateBytes64(
            ConcatenateBytes64(challenge_4, challenge_5),
            ConcatenateBytes64(challenge_6, challenge_7)
        )
    );
    
    result == RSI_SUCCESS
    && size == AttestationTokenMaxSize(realm)
    && new_s.recs[rec].attest_state == ATTEST_IN_PROGRESS
    && new_s.recs[rec].attest_challenge == expected_challenge
    && (forall addr: Address :: 
        addr != rec ==> new_s.recs[addr] == old_s.recs[addr])
}
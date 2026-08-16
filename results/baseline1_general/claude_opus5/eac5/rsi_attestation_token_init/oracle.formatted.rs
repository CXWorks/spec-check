pub open spec fn rsi_attestation_token_init_spec(
    challenge_0: Bits64,
    challenge_1: Bits64,
    challenge_2: Bits64,
    challenge_3: Bits64,
    challenge_4: Bits64,
    challenge_5: Bits64,
    challenge_6: Bits64,
    challenge_7: Bits64,
    result: RsiCommandReturnCode,
    size: UInt64,
    old_s: S,
    new_s: S,
) -> bool {
    (result == RSI_SUCCESS ==> CurrentRec(new_s).attest_state == ATTEST_IN_PROGRESS) && (result
        == RSI_SUCCESS ==> CurrentRec(new_s).attest_challenge == [
        challenge_0,
        challenge_1,
        challenge_2,
        challenge_3,
        challenge_4,
        challenge_5,
        challenge_6,
        challenge_7,
    ]) && (result != RSI_SUCCESS ==> CurrentRec(new_s).attest_state == CurrentRec(
        old_s,
    ).attest_state) && (result != RSI_SUCCESS ==> CurrentRec(new_s).attest_challenge == CurrentRec(
        old_s,
    ).attest_challenge)
}
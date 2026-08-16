pub open spec fn rsi_attestation_token_init_spec(result: RsiCommandReturnCode, challenge_0: Bits64, challenge_1: Bits64, challenge_2: Bits64, challenge_3: Bits64, challenge_4: Bits64, challenge_5: Bits64, challenge_6: Bits64, challenge_7: Bits64, old_s: S, new_s: S) -> bool {
    result == RSI_SUCCESS
    && CurrentRec(new_s).attest_state == ATTEST_IN_PROGRESS
    && CurrentRec(new_s).attest_challenge[0] == challenge_0
    && CurrentRec(new_s).attest_challenge[1] == challenge_1
    && CurrentRec(new_s).attest_challenge[2] == challenge_2
    && CurrentRec(new_s).attest_challenge[3] == challenge_3
    && CurrentRec(new_s).attest_challenge[4] == challenge_4
    && CurrentRec(new_s).attest_challenge[5] == challenge_5
    && CurrentRec(new_s).attest_challenge[6] == challenge_6
    && CurrentRec(new_s).attest_challenge[7] == challenge_7
}
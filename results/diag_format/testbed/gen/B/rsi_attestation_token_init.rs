pub open spec fn rsi_attestation_token_init_spec(challenge_0: Bits64, challenge_1: Bits64, challenge_2: Bits64, challenge_3: Bits64, challenge_4: Bits64, challenge_5: Bits64, challenge_6: Bits64, challenge_7: Bits64, result: RsiCommandReturnCode, size: UInt64, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (new_s.CurrentRec().attest_state == ATTEST_IN_PROGRESS)
    && (new_s.CurrentRec().attest_challenge[0] == challenge_0)
    && (new_s.CurrentRec().attest_challenge[1] == challenge_1)
    && (new_s.CurrentRec().attest_challenge[2] == challenge_2)
    && (new_s.CurrentRec().attest_challenge[3] == challenge_3)
    && (new_s.CurrentRec().attest_challenge[4] == challenge_4)
    && (new_s.CurrentRec().attest_challenge[5] == challenge_5)
    && (new_s.CurrentRec().attest_challenge[6] == challenge_6)
    && (new_s.CurrentRec().attest_challenge[7] == challenge_7)
    && (size == new_s.CurrentRealm().attest_token_max_size)
}
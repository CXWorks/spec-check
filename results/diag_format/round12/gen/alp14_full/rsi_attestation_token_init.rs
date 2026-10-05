pub open spec fn rsi_attestation_token_init_spec(challenge_0: UInt64, challenge_1: UInt64, challenge_2: UInt64, challenge_3: UInt64, challenge_4: UInt64, challenge_5: UInt64, challenge_6: UInt64, challenge_7: UInt64, result: RsiCommandReturnCode, size: UInt64, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> RecAttestationState(new_s) == ATTEST_IN_PROGRESS)
  && (result == RSI_SUCCESS ==> RecAttestationChallenge(new_s) == [challenge_0, challenge_1, challenge_2, challenge_3, challenge_4, challenge_5, challenge_6, challenge_7])
  && (result == RSI_SUCCESS ==> size == size)
  && ((!(result == RSI_SUCCESS))
    ==> RecAttestationState(new_s) == RecAttestationState(old_s))
  && ((!(result == RSI_SUCCESS))
    ==> RecAttestationChallenge(new_s) == RecAttestationChallenge(old_s))
}
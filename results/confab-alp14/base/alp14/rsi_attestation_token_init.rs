pub open spec fn rsi_attestation_token_init_spec(result: RsiCommandReturnCode, challenge_0: u64, challenge_1: u64, challenge_2: u64, challenge_3: u64, challenge_4: u64, challenge_5: u64, challenge_6: u64, challenge_7: u64, size: u64, old_s: S, new_s: S) -> bool {
    result == RSI_SUCCESS
}
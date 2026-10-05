pub open spec fn vci_certification_token_init_spec(challenge_0: Bits64, challenge_1: Bits64, challenge_2: Bits64, challenge_3: Bits64, challenge_4: Bits64, challenge_5: Bits64, challenge_6: Bits64, challenge_7: Bits64, result: Vcicommandreturncode, size: UInt64, old_s: S, new_s: S) -> bool {
  (result == VCI_SUCCESS ==> Currentworker(new_s).certify_state == CERTIFY_IN_PROGRESS)
  && (result == VCI_SUCCESS ==> Currentworker(new_s).certify_challenge == (challenge_0 << 64 | challenge_1 << 64 | challenge_2 << 64 | challenge_3 << 64 | challenge_4 << 64 | challenge_5 << 64 | challenge_6 << 64 | challenge_7))
  && (result == VCI_SUCCESS ==> size == size)
  && ((!(result == VCI_SUCCESS))
    ==> Currentworker(new_s).certify_state == Currentworker(old_s).certify_state)
  && ((!(result == VCI_SUCCESS))
    ==> Currentworker(new_s).certify_challenge == Currentworker(old_s).certify_challenge)
  && (result != VCI_SUCCESS
    ==> size == 0)
}
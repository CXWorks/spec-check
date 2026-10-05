pub open spec fn pinctrl_release__3_11_2_10_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!Bits64(old_s.cmd_input_flags, 31, 2) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!Bits64(old_s.cmd_input_flags, 1, 0) > 1 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidPinOrGroup(old_s.cmd_input_identifier, Bits64(old_s.cmd_input_flags, 1, 0)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> !HasExclusiveControl(CallingAgent(), old_s.cmd_input_identifier, Bits64(old_s.cmd_input_flags, 1, 0)))
}
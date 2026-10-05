pub open spec fn pinctrl_request__3_11_2_9_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (Bits64(old_s.cmd_input_1 as u64, 31, 2) != 0 ==> result == RSI_ERROR_INPUT)
    && (Bits64(old_s.cmd_input_1 as u64, 1, 0) != 0 && Bits64(old_s.cmd_input_1 as u64, 1, 0) != 1 ==> result == RSI_ERROR_INPUT)
    && (!IsValidPinOrGroup(old_s.cmd_input_0 as u32, Bits64(old_s.cmd_input_1 as u64, 1, 0)) ==> result == RSI_ERROR_NOT_FOUND)
    && (!AgentMayRequestPinOrGroup(CallingAgent(), old_s.cmd_input_0 as u32, Bits64(old_s.cmd_input_1 as u64, 1, 0)) ==> result == RSI_ERROR_DENIED)
    && (IsUnderExclusiveControlOfOtherAgent(old_s.cmd_input_0 as u32, Bits64(old_s.cmd_input_1 as u64, 1, 0), CallingAgent()) ==> result == RSI_ERROR_IN_USE)
    && (result == RSI_SUCCESS ==> PinOrGroupAt(old_s.cmd_input_0 as u32, Bits64(old_s.cmd_input_1 as u64, 1, 0)).owner == CallingAgent())
    && (result == RSI_SUCCESS ==> forall|a: AgentId| a != CallingAgent() ==> !PinOrGroupAvailableTo(a, old_s.cmd_input_0 as u32, Bits64(old_s.cmd_input_1 as u64, 1, 0)))
}
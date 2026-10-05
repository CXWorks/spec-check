pub open spec fn pinctrl_request__3_11_2_9_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (flags_rsvd(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_selector(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (id_valid(old_s) ==> ResultEqual(result, NOT_FOUND))
    && (agent_permitted(old_s) ==> ResultEqual(result, DENIED))
    && (in_use(old_s) ==> ResultEqual(result, IN_USE))
    && (ResultEqual(result, SUCCESS) ==> owner(old_s, new_s))
}

fn flags_rsvd(old_s: S) -> bool {
    old_s.cmd_input_param_1[31..2] != 0
}

fn flags_selector(old_s: S) -> bool {
    let flags = old_s.cmd_input_param_1;
    (flags[1..0] != 0) && (flags[1..0] != 1)
}

fn id_valid(old_s: S) -> bool {
    !IsValidPinOrGroup(old_s.cmd_input_param_0, old_s.cmd_input_param_1[1..0])
}

fn agent_permitted(old_s: S) -> bool {
    !AgentMayRequestPinOrGroup(old_s.cmd_input_caller, old_s.cmd_input_param_0, old_s.cmd_input_param_1[1..0])
}

fn in_use(old_s: S) -> bool {
    IsUnderExclusiveControl(old_s.cmd_input_param_0, old_s.cmd_input_param_1[1..0])
    && ExclusiveOwner(old_s.cmd_input_param_0, old_s.cmd_input_param_1[1..0]) != old_s.cmd_input_caller
}

fn owner(old_s: S, new_s: S) -> bool {
    ExclusiveOwner(new_s.cmd_input_param_0, new_s.cmd_input_param_1[1..0]) == new_s.cmd_input_caller
}
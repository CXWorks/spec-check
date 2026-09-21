pub open spec fn 3.11.2.10_pinctrl_release_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (new_s == old_s))
    && (result == SCMI_NOT_FOUND ==> (new_s == old_s))
    && (result == SCMI_INVALID_PARAMETERS ==> (new_s == old_s))
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_INVALID_PARAMETERS ==> (new_s == old_s))
}
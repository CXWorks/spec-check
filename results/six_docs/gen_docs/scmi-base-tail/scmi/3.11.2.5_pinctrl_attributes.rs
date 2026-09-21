pub open spec fn pinctrl_attributes_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s == new_s))
    && (result == SCMI_NOT_FOUND ==> (old_s == new_s))
    && (result == SCMI_SUCCESS ==> (old_s == new_s))
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_INVALID_PARAMETERS ==> (old_s == new_s))
}
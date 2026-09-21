pub open spec fn pinctrl_name_get_spec(result: int32, name: [uint8; 64], flags: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_NOT_FOUND ==> flags == 0 && name[0] == 0)
    && (result == SCMI_SUCCESS ==> flags == 0 && name[0] != 0)
    && (result != SCMI_SUCCESS ==> flags == 0 && name[0] == 0)
}
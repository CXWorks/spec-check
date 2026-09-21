pub open spec fn pinctrl_name_get_spec(result: int32, flags: uint32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == 0 ==> (flags == 0 && name[0] == 0))
    && (result != 0 ==> flags == 0)
}
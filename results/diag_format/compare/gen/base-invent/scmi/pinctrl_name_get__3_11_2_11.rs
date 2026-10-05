pub open spec fn pinctrl_name_get__3_11_2_11_spec(result: int32, flags: uint32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> flags == 0 && name == [0u8; 64])
    && (result == SUCCESS ==> flags == 0 && name[0] != 0)
    && (result != SUCCESS && result != NOT_FOUND && result != SUCCESS ==> true)
    && (flags != 0 ==> result == NOT_FOUND)
    && (name[0] == 0 && result != NOT_FOUND ==> result == SUCCESS)
}
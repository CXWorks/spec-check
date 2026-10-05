pub open spec fn clock_name_get__3_6_2_11_spec(result: int32, flags: uint32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (flags == 0 && name == [0u8; 64]))
    && (result == SUCCESS ==> (flags == 0 && name[0] == 0u8))
    && (result != SUCCESS && result != NOT_FOUND && result != SUCCESS ==> (flags == 0 && name == [0u8; 64]))
}
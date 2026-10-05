pub open spec fn reset_domain_name_get__3_8_2_8_spec(result: int32, flags: uint32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (flags == 0 && name == [0u8; 64]))
    && (result == SUCCESS ==> (flags == 0 && name[0] == 0u8 && name[1..64] == name[1..64]))
    && (result != NOT_FOUND && result != SUCCESS ==> (flags == 0 && name == [0u8; 64]))
}
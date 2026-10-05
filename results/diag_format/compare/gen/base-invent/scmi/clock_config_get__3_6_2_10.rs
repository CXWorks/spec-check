pub open spec fn clock_config_get__3_6_2_10_spec(result: int32, attributes: uint32, config: uint32, extended_config_val: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> clock_id_not_found(old_s, clock_id))
    && (result == INVALID_PARAMETERS ==> (flags_nonzero(old_s, flags) || extended_config_type_unused(old_s, extended_config_type)))
    && (result == SUCCESS ==> (attributes == old_s.attributes && config == old_s.config && extended_config_val == old_s.extended_config_val))
}

fn clock_id_not_found(old_s: S, clock_id: uint32) -> bool {
    true
}

fn flags_nonzero(old_s: S, flags: uint32) -> bool {
    (flags & 0xFF00) != 0
}

fn extended_config_type_unused(old_s: S, extended_config_type: uint32) -> bool {
    extended_config_type != 0
}
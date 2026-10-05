pub open spec fn power_domain_name_get__3_3_2_10_spec(result: int32, flags: uint32, ext_name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (old_s.power_domains[old_s.domain_id as int].is_none()))
    && (result == SUCCESS ==> (old_s.power_domains[old_s.domain_id as int].is_some() && new_s.power_domains == old_s.power_domains && new_s.flags == 0 && new_s.ext_name == ext_name))
    && (result != SUCCESS && result != NOT_FOUND ==> true)
    && (flags != 0 ==> false)
    && (ext_name[63] != 0 ==> false)
}
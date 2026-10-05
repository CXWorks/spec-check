pub open spec fn performance_domain_name_get__3_5_6_19_spec(result: int32, name: [u8; 64], flags: u32, old_s: S, new_s: S) -> bool {
    (result == 0x13_00000000 ==> (flags == 0 && name[0] == 0))
    && (result == 0x13_00000001 ==> (flags == 0 && name[0] == 0))
    && (result != 0x13_00000000 && result != 0x13_00000001 ==> (flags == 0 && name[0] == 0))
    && (result == 0x13_00000000 ==> (new_s.performance_domains[old_s.domain_id as usize].name == name))
    && (result == 0x13_00000001 ==> (new_s.performance_domains[old_s.domain_id as usize].name == name))
    && (result != 0x13_00000000 && result != 0x13_00000001 ==> (new_s.performance_domains[old_s.domain_id as usize].name == name))
}
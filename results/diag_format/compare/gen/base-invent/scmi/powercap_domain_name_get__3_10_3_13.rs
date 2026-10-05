pub open spec fn powercap_domain_name_get__3_10_3_13_spec(result: int32, flags: uint32, name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == 0 ==> (flags == 0 && name[0] == 0))
    && (result != 0 ==> (flags == 0 && name[0] == 0))
    && (result == 0 ==> (old_s.powercap_domains[old_s.domain_id] == new_s.domain_id))
    && (result != 0 ==> (old_s.powercap_domains[old_s.domain_id] == new_s.domain_id))
}
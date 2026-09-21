pub open spec fn 3.3.2.10_power_domain_name_get_spec(result: int32, flags: uint32, ext_name: [uint8; 64], old_s: S, new_s: S) -> bool {
    (result == 0 ==> (flags == 0 && ext_name[63] == 0))
    && (result != 0 ==> (flags == 0 && ext_name[63] == 0))
}
pub open spec fn 3.5.6.19_performance_domain_name_get_spec(result: int32, name: [u8; 64], flags: u32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (flags == 0 && name[0] == 0))
    && (result != 0 ==> (flags == 0 && name[0] == 0))
    && (result == 0 ==> true)
    && (result != 0 ==> true)
}
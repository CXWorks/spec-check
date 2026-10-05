pub open spec fn powercap_cap_get__3_10_3_7_spec(result: int32, power_cap: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> (domain_id(old_s) != 0 || cpli(old_s) != 0))
    && (result == NOT_SUPPORTED ==> true)
    && (result == SUCCESS ==> (power_cap(new_s) == power_cap(old_s)))
}
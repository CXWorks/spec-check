pub open spec fn 3.10.3.9_powercap_mai_get_spec(result: int32, mai: uint32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> mai == old_s.powercap_mai(old_s, domain_id))
    && (result != 0 ==> true)
}
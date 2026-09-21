pub open spec fn 3.10.3.8_powercap_cap_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!((old_s.domain_id as int) < 0) ==> ResultEqual(result, NOT_FOUND))
    && (!((old_s.cpli as int) < 0) ==> ResultEqual(result, NOT_FOUND))
    && (!((old_s.flags as int) & 0x3 != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!((old_s.power_cap as int) < 0) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((ResultEqual(result, SUCCESS) || ResultEqual(result, NOT_FOUND) || ResultEqual(result, NOT_SUPPORTED) || ResultEqual(result, INVALID_PARAMETERS) || ResultEqual(result, DENIED)) ==> true)
}
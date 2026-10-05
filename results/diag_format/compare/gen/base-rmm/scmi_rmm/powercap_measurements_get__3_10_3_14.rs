pub open spec fn powercap_measurements_get__3_10_3_14_spec(result: int32, power: uint32, mai: uint32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsPowercapMeasurementsGetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetPowercapMeasurements(old_s, caller, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (power == PowercapDomain(old_s, domain_id).average_power_over_latest_mai && mai == PowercapDomain(old_s, domain_id).mai))
    && (old_s == new_s)
}
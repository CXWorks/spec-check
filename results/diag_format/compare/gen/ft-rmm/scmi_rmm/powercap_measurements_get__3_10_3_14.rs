pub open spec fn powercap_measurements_get__3_10_3_14_spec(domain_id: uint32, status: int32, power: uint32, mai: uint32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsPowercapMeasurementsGetSupported(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetPowercapMeasurements(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> power == PowercapDomain(new_s, domain_id).average_power_over_latest_mai)
  && (ResultEqual(status, SUCCESS) ==> mai == PowercapDomain(new_s, domain_id).mai)
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsPowercapMeasurementsGetSupported(old_s, domain_id) &&
       AgentMayGetPowercapMeasurements(old_s, caller, domain_id))
    ==> ResultEqual(status, SUCCESS))
}
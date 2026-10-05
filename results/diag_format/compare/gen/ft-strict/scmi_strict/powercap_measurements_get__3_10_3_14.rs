pub open spec fn powercap_measurements_get__3_10_3_14_spec(domain_id: UInt32, status: Int32, power: UInt32, mai: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsPowercapMeasurementsGetSupported(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetPowercapMeasurements(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> power == AveragePowerOverLatestInterval(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> mai == PowercapMeasurementAveragingInterval(new_s, domain_id))
  && ((IsValidPowercapDomain(old_s, domain_id) &&
       IsPowercapMeasurementsGetSupported(old_s, domain_id) &&
       AgentMayGetPowercapMeasurements(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> power == AveragePowerOverLatestInterval(old_s, domain_id))
  && (result != SUCCESS
    ==> mai == PowercapMeasurementAveragingInterval(old_s, domain_id))
}
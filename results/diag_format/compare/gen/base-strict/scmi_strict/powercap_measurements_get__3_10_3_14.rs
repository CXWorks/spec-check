pub open spec fn powercap_measurements_get__3_10_3_14_spec(
    result: Int32,
    power: UInt32,
    mai: UInt32,
    old_s: S,
    new_s: S,
    domain_id: UInt32,
    calling_agent: Agent,
) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsPowercapMeasurementsGetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetPowercapMeasurements(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (power == AveragePowerOverLatestInterval(old_s, domain_id) && mai == PowercapMeasurementAveragingInterval(old_s, domain_id)))
    && (ResultEqual(result, SUCCESS) ==> old_s == new_s)
}
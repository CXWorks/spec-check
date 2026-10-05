pub open spec fn powercap_measurements_get__3_10_3_14_spec(domain_id: u32, status: i32, power: u32, mai: u32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainIsValid(old_s, domain_id) ==> status == NOT_FOUND)
    && ((PowercapDomainIsValid(old_s, domain_id)
        && !PowercapMeasurementsSupported(old_s, domain_id)) ==> status == NOT_SUPPORTED)
    && ((PowercapDomainIsValid(old_s, domain_id)
        && PowercapMeasurementsSupported(old_s, domain_id)
        && !PowercapAgentAllowedMeasurements(old_s, domain_id)) ==> status == DENIED)
    && ((PowercapDomainIsValid(old_s, domain_id)
        && PowercapMeasurementsSupported(old_s, domain_id)
        && PowercapAgentAllowedMeasurements(old_s, domain_id)) ==> (
            status == SUCCESS
            && power == PowercapAveragePower(old_s, domain_id)
            && mai == PowercapMai(old_s, domain_id)
        ))
    && (status == SUCCESS ==> (
            PowercapDomainIsValid(old_s, domain_id)
            && PowercapMeasurementsSupported(old_s, domain_id)
            && PowercapAgentAllowedMeasurements(old_s, domain_id)
            && power == PowercapAveragePower(old_s, domain_id)
            && mai == PowercapMai(old_s, domain_id)
        ))
    && new_s == old_s
}

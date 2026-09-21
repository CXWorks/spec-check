pub open spec fn 3.5.6.13_reduce_sustained_performance_level_spec(result: int32, old_s: S, new_s: S) -> bool {
    let domain_id: UInt32 = old_s.cmd_input_0;
    let sustained_level: UInt32 = old_s.cmd_input_1;
    let domain_attrs: PerformanceDomainAttributes = old_s.performance_domain_attributes;
    let domain_id_valid: bool = domain_id < old_s.performance_domain_count;
    let domain_is_qos_only: bool = (domain_attrs.attributes & (1u32 << 23)) != 0;
    let platform_sustained_level: UInt32 = domain_attrs.sustained_perf_level;
    let requested_level_valid: bool = sustained_level <= platform_sustained_level;
    let command_supported: bool = true; // Command is optional, assume supported if not explicitly denied
    let status_success: bool = result == 0;
    let status_not_found: bool = result == 1;
    let status_not_supported: bool = result == 2;
    let status_denied: bool = result == 3;
    let status_out_of_range: bool = result == 4;

    (!domain_id_valid ==> status_not_found)
    && (!command_supported ==> status_not_supported)
    && (!requested_level_valid ==> status_out_of_range)
    && (!domain_is_qos_only ==> status_denied)
    && (status_success ==> (new_s.performance_domain_attributes == old_s.performance_domain_attributes))
}
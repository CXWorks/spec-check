pub open spec fn performance_domain_attributes__3_5_6_5_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(domain_id(old_s)) ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (
        status(old_s) == SUCCESS
        && attributes(old_s)[21..0] == 0
        && (attributes(old_s)[23] == 1 ==> attributes(old_s)[31..28] == 0)
        && (attributes(old_s)[23] == 1 ==> attributes(old_s)[22] == 0)
        && rate_limit(old_s)[31..20] == 0
        && (qos_capability_types(old_s)[31..24] == 0 && qos_capability_types(old_s)[15..8] == 0)
    ))
}
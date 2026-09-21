pub open spec fn 3.5.8.1_performance_limits_changed_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> true)
    && (result == RSI_ERROR_INPUT ==> (old_s.agent_id != new_s.agent_id || old_s.domain_id != new_s.domain_id || old_s.range_max != new_s.range_max || old_s.range_min != new_s.range_min))
    && (result == RSI_ERROR_STATE ==> (old_s.agent_id != new_s.agent_id || old_s.domain_id != new_s.domain_id || old_s.range_max != new_s.range_max || old_s.range_min != new_s.range_min))
    && (result == RSI_INCOMPLETE ==> (old_s.agent_id != new_s.agent_id || old_s.domain_id != new_s.domain_id || old_s.range_max != new_s.range_max || old_s.range_min != new_s.range_min))
    && (result == RSI_ERROR_UNKNOWN ==> (old_s.agent_id != new_s.agent_id || old_s.domain_id != new_s.domain_id || old_s.range_max != new_s.range_max || old_s.range_min != new_s.range_min))
}
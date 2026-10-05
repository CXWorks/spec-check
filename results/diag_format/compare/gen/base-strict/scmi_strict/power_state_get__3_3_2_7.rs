pub open spec fn power_state_get__3_3_2_7_spec(result: Int32, power_state: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPowerDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (ResultEqual(result, SUCCESS) && ReportsCurrentPowerState(domain_id(old_s), power_state) && (IsDevicePowerDomain(domain_id(old_s)) ==> IsDevicePowerStateEncoding(power_state))))
    && (old_s == new_s)
}
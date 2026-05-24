pub open spec fn psci_system_reset_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    result.is_Ok() && CurrentRealm(new_s).state == RmmGranuleState::RealmsSystemOff
}
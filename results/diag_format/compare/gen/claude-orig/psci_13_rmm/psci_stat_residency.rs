pub open spec fn psci_stat_residency_spec(fid: UInt32, target_cpu: Mpidr, power_state: PowerState, result: UInt64, old_s: S, new_s: S) -> bool {
    (!StatFunctionsImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsPresentNode(target_cpu) ==> ResultEqual(result, 0))
    && (!NodeSupportsState(StatNode(target_cpu, power_state), StatLocalState(power_state)) ==> ResultEqual(result, 0))
    && ((StatFunctionsImplemented()
        && IsPresentNode(target_cpu)
        && NodeSupportsState(StatNode(target_cpu, power_state), StatLocalState(power_state)))
        ==> (
            StatNode(target_cpu, power_state) == NodeAt(target_cpu, HighestPowerLevel(power_state))
            && (IsStatCount(fid) ==> ResultEqual(result, ((StateUseCount(StatNode(target_cpu, power_state), StatLocalState(power_state)) as int) % (pow2(ResultBits(fid) as nat) as int)) as UInt64))
            && (IsStatResidency(fid) ==> ResultEqual(result, ((StateResidencyUs(StatNode(target_cpu, power_state), StatLocalState(power_state)) as int) % (pow2(ResultBits(fid) as nat) as int)) as UInt64))
        ))
    && new_s == old_s
}

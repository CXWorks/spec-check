pub open spec fn cpu_on_spec(target_cpu: MPIDR, entry_point_address: Address, context_id: UInt64, result: Result<(), PsciReturnCode>, old_s: S, new_s: S) -> bool {
    (!IsValidMpidr(target_cpu) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsKnownUnavailableToCaller(entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
    && (CoreAt(old_s, target_cpu).state == ON ==> ResultEqual(result, ALREADY_ON))
    && (CoreAt(old_s, target_cpu).state == ON_PENDING ==> ResultEqual(result, ON_PENDING))
    && (!CanPowerUpPhysically(target_cpu) ==> ResultEqual(result, INTERNAL_FAILURE))
    && (!IsAvailableForOsUse(target_cpu) ==> ResultEqual(result, DENIED))
    && ((IsValidMpidr(target_cpu)
        && !IsKnownUnavailableToCaller(entry_point_address)
        && CoreAt(old_s, target_cpu).state != ON
        && CoreAt(old_s, target_cpu).state != ON_PENDING
        && CanPowerUpPhysically(target_cpu)
        && IsAvailableForOsUse(target_cpu))
        ==> (result.is_Ok()
            && ContextIdRegister(CoreAt(new_s, target_cpu)) == context_id
            && CachesInvalidatedOnBoot(target_cpu)
            && CoherencyManaged(target_cpu)))
}

pub open spec fn cpu_on_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, target_cpu: MPIDR, entry_point_address: Address, context_id: u64) -> bool {
    (!IsValidMpidr(target_cpu) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsKnownUnavailableToCaller(entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
    && (CoreAt(old_s, target_cpu).state == ON ==> ResultEqual(result, ALREADY_ON))
    && (CoreAt(old_s, target_cpu).state == ON_PENDING ==> ResultEqual(result, ON_PENDING))
    && (!CanPowerUpPhysically(target_cpu) ==> ResultEqual(result, INTERNAL_FAILURE))
    && (!IsAvailableForOsUse(target_cpu) ==> ResultEqual(result, DENIED))
    && (result.is_Ok() ==> (CoreAt(new_s, target_cpu).state == ON))
    && (result.is_Ok() ==> CoreAt(new_s, target_cpu).power == true)
    && (result.is_Ok() ==> CoreAt(new_s, target_cpu).caches == old_s.core_caches)
}
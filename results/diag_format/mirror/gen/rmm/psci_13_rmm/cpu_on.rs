pub open spec fn cpu_on_spec(target_cpu: MPIDR, entry_point_address: Address, context_id: int, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsValidMpidr(old_s, target_cpu) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsKnownUnavailableToCaller(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (CoreAt(old_s, target_cpu).state == ON ==> ResultEqual(result, ALREADY_ON))
  && (CoreAt(old_s, target_cpu).state == ON_PENDING ==> ResultEqual(result, ON_PENDING))
  && (!CanPowerUpPhysically(old_s, target_cpu) ==> ResultEqual(result, INTERNAL_FAILURE))
  && (!IsAvailableForOsUse(old_s, target_cpu) ==> ResultEqual(result, DENIED))
  && (result.is_Ok() ==> CoreAt(new_s, target_cpu) restarts execution at entry_point_address in the return Exception level)
  && (result.is_Ok() ==> ContextIdRegister(new_s, CoreAt(new_s, target_cpu)) == context_id)
  && (result.is_Ok() ==> CachesInvalidatedOnBoot(new_s, target_cpu))
  && (result.is_Ok() ==> CoherencyManaged(new_s, target_cpu))
  && ((IsValidMpidr(old_s, target_cpu) &&
       !IsKnownUnavailableToCaller(old_s, entry_point_address) &&
       !(CoreAt(old_s, target_cpu).state == ON) &&
       !(CoreAt(old_s, target_cpu).state == ON_PENDING) &&
       CanPowerUpPhysically(old_s, target_cpu) &&
       IsAvailableForOsUse(old_s, target_cpu))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> CoreAt(new_s, target_cpu).state == CoreAt(old_s, target_cpu).state)
  && (result.is_Err()
    ==> CoreAt(new_s, target_cpu).power == CoreAt(old_s, target_cpu).power)
  && (result.is_Err()
    ==> CoreAt(new_s, target_cpu).caches == CoreAt(old_s, target_cpu).caches)
}
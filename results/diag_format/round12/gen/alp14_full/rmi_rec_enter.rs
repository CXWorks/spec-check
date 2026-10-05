pub open spec fn rmi_rec_enter_spec(rec_ptr: PhysInt, run_ptr: PhysInt, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((rec_ptr % GRANULE_SIZE) == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).pas == PAS_NS ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((rec_ptr % GRANULE_SIZE) == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state == REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, run_ptr).pas == PAS_NS ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((run_ptr % GRANULE_SIZE) == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RealmAt(new_s, RealmAt(new_s, rec_ptr).realm).state != REALM_NEW)
  && (result.is_Ok() ==> RealmAt(new_s, RealmAt(new_s, rec_ptr).realm).state != REALM_SYSTEM_OFF)
  && (result.is_Ok() ==> RealmAt(new_s, RealmAt(new_s, rec_ptr).realm).state != REALM_SYSTEM_OFF)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).state != REC_RUNNING)
  && (result.is_Ok() ==> !(RecAt(new_s, rec_ptr).not_runnable))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).pending == REC_PENDING_NONE)
  && ((!( (rec_ptr % GRANULE_SIZE) == 0) &&
       !(GranuleAt(old_s, rec_ptr).pas == PAS_NS) &&
       !is_delegable_physical_address(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state == REC) &&
       !(GranuleAt(old_s, run_ptr).pas == PAS_NS) &&
       !((run_ptr % GRANULE_SIZE) == 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).emulatable_abort == RecAt(old_s, rec_ptr).emulatable_abort)
}
pub open spec fn rmi_rec_enter_spec(run_ptr: Address, rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((run_ptr % GRANULE_SIZE) == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, run_ptr).pas == PAS_NS ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((rec_ptr % GRANULE_SIZE) == 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegable_physical_address(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state == REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM(1)))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).not_runnable ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecRunAt(old_s, run_ptr).emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecRunAt(old_s, run_ptr).gicv3_hcr != 0 || RecRunAt(old_s, run_ptr).gicv3_lrs != 0 ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).pending != REC_PENDING_NONE ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).emulatable_abort == RecAt(new_s, rec_ptr).emulatable_abort + 1)
  && ((!( (run_ptr % GRANULE_SIZE) == 0) &&
       !(GranuleAt(old_s, run_ptr).pas == PAS_NS) &&
       !((rec_ptr % GRANULE_SIZE) == 0) &&
       !(is_delegable_physical_address(old_s, rec_ptr)) &&
       !(GranuleAt(old_s, rec_ptr).state == REC) &&
       !(RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_NEW) &&
       !(RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_SYSTEM_OFF) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).not_runnable) &&
       !(RecRunAt(old_s, run_ptr).emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT) &&
       !(RecRunAt(old_s, run_ptr).gicv3_hcr != 0 || RecRunAt(old_s, run_ptr).gicv3_lrs != 0) &&
       !(RecAt(old_s, rec_ptr).pending != REC_PENDING_NONE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).emulatable_abort == RecAt(old_s, rec_ptr).emulatable_abort)
}
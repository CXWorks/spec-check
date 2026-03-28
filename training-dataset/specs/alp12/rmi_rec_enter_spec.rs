pub open spec fn rmi_rec_enter_spec(rec_ptr: Address, run_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, run_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0 as int)))
  && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM(1 as int)))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).flags.runnable == NOT_RUNNABLE ==> ResultEqual(result, RMI_ERROR_REC))
  && ((RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT) ==> ResultEqual(result, RMI_ERROR_REC))
  && (!Gicv3ConfigIsValid(old_s, RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr, RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs) ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).pending != REC_PENDING_NONE ==> ResultEqual(result, RMI_ERROR_REC))
  && ((AddrIsGranuleAligned(old_s, run_ptr) &&
       GranuleAccessPermitted(old_s, run_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_NEW) &&
       !(RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_SYSTEM_OFF) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).flags.runnable == NOT_RUNNABLE) &&
       !((RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT)) &&
       Gicv3ConfigIsValid(old_s, RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr, RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs) &&
       !(RecAt(old_s, rec_ptr).pending != REC_PENDING_NONE))
    ==> result.is_Ok())
}

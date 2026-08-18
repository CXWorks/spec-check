pub open spec fn rmi_rec_enter_spec(rec: Address, run_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, run_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(0, new_s).state == REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (Realm(0, new_s).state == REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM(1)))
  && (Rec(0, new_s).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (Rec(0, new_s).flags.runnable == NOT_RUNNABLE ==> ResultEqual(result, RMI_ERROR_REC))
  && ((Rec(0, new_s).enter.flags.emul_mmio == RMI_EMULATED_MMIO && Rec(0, new_s).emulatable_abort != EMULATABLE_ABORT) ==> ResultEqual(result, RMI_ERROR_REC))
  && (!Gicv3ConfigIsValid(0, new_s, Rec(0, new_s).enter.gicv3_hcr, Rec(0, new_s).enter.gicv3_lrs) ==> ResultEqual(result, RMI_ERROR_REC))
  && (Rec(0, new_s).psci_pending == PSCI_REQUEST_PENDING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> Rec(0, new_s).emulatable_abort == Rec(0, new_s).emulatable_abort)
  && ((AddrIsGranuleAligned(old_s, run_ptr) &&
       PaIsDelegable(old_s, run_ptr) &&
       GranuleAccessPermitted(old_s, run_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, rec) &&
       PaIsDelegable(old_s, rec) &&
       !(Granule(old_s, rec).state != REC) &&
       !(Realm(0, old_s).state == REALM_NEW) &&
       !(Realm(0, old_s).state == REALM_SYSTEM_OFF) &&
       !(Rec(0, old_s).state == REC_RUNNING) &&
       !(Rec(0, old_s).flags.runnable == NOT_RUNNABLE) &&
       !((Rec(0, old_s).enter.flags.emul_mmio == RMI_EMULATED_MMIO && Rec(0, old_s).emulatable_abort != EMULATABLE_ABORT)) &&
       Gicv3ConfigIsValid(0, old_s, Rec(0, old_s).enter.gicv3_hcr, Rec(0, old_s).enter.gicv3_lrs) &&
       !(Rec(0, old_s).psci_pending == PSCI_REQUEST_PENDING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rec(0, new_s).emulatable_abort == Rec(0, new_s).emulatable_abort)
}
pub open spec fn rmi_rec_enter_spec(rec_ptr: Address, run_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, run_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, run_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, rec_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rec_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rec_ptr).state != REC ==> result.status == RMI_ERROR_INPUT)
  && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state != REALM_ACTIVE ==> result.status == RMI_ERROR_REALM(0))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> result.status == RMI_ERROR_REC(0))
  && (RecAt(old_s, rec_ptr).flags.runnable == NOT_RUNNABLE ==> result.status == RMI_ERROR_REC(0))
  && ((RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT) ==> result.status == RMI_ERROR_REC(0))
  && (!Gicv3ConfigIsValid(old_s, 0, []) ==> result.status == RMI_ERROR_REC(0))
  && (RecAt(old_s, rec_ptr).pending == REC_PENDING_PSCI ==> result.status == RMI_ERROR_REC(0))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).emulatable_abort == EMULATABLE_ABORT)
  && ((AddrIsRmiGranuleAligned(old_s, run_ptr) &&
       NonSecureAccessPermitted(old_s, run_ptr) &&
       AddrIsRmiGranuleAligned(old_s, rec_ptr) &&
       PaIsTracked(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state != REALM_ACTIVE) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).flags.runnable == NOT_RUNNABLE) &&
       !((RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT)) &&
       Gicv3ConfigIsValid(old_s, 0, []))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).emulatable_abort == RecAt(old_s, rec_ptr).emulatable_abort)
}
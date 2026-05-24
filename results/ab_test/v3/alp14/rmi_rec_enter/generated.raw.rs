pub open spec fn rmi_rec_enter_spec(result: Result<(), RmiStatusCode>, run_ptr: Address, rec_ptr: Address, old_s: S, new_s: S) -> bool {
    // Failure conditions (input validation order)
    (!AddrIsGranuleAligned(run_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(run_ptr, PAS_NS) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != RmmGranuleState::REC ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    // Failure conditions (realm/rec state checks)
    && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == RmmRealmState::REALM_NEW ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REALM))
    && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == RmmRealmState::REALM_SYSTEM_OFF ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REALM))
    && (RecAt(old_s, rec_ptr).state == RmmRecState::REC_RUNNING ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REC))
    && (RecAt(old_s, rec_ptr).flags.runnable == RmmRecRunnable::NOT_RUNNABLE ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REC))
    && ((RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RmiEmulatedMmio::RMI_EMULATED_MMIO
         && RecAt(old_s, rec_ptr).emulatable_abort != RmmEmulatableAbort::EMULATABLE_ABORT)
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REC))
    && (!Gicv3ConfigIsValid(old_s, RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr, RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs)
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REC))
    && (RecAt(old_s, rec_ptr).pending != RmmRecPending::REC_PENDING_NONE ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_REC))
    // Success condition
    && ((AddrIsGranuleAligned(run_ptr)
         && GranuleAccessPermitted(run_ptr, PAS_NS)
         && AddrIsGranuleAligned(rec_ptr)
         && PaIsDelegable(rec_ptr)
         && GranuleAt(old_s, rec_ptr).state == RmmGranuleState::REC
         && RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state != RmmRealmState::REALM_NEW
         && RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state != RmmRealmState::REALM_SYSTEM_OFF
         && RecAt(old_s, rec_ptr).state != RmmRecState::REC_RUNNING
         && RecAt(old_s, rec_ptr).flags.runnable != RmmRecRunnable::NOT_RUNNABLE
         && !(RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio == RmiEmulatedMmio::RMI_EMULATED_MMIO
              && RecAt(old_s, rec_ptr).emulatable_abort != RmmEmulatableAbort::EMULATABLE_ABORT)
         && Gicv3ConfigIsValid(old_s, RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr, RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs)
         && RecAt(old_s, rec_ptr).pending == RmmRecPending::REC_PENDING_NONE)
        ==> (result.is_Ok()
             && RecAt(new_s, rec_ptr).emulatable_abort == RecAt(old_s, rec_ptr).emulatable_abort))
}
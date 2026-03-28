pub open spec fn rmi_rec_create_spec(rd: Address, rec_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RealmAt(old_s, rd).num_recs == pow2(ImplFeatures(old_s).max_recs_order as nat) - 1 ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (RecIndex(old_s, RmiRecParamsAt(old_s, params_ptr).mpidr) != RealmAt(old_s, rd).rec_index ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiRecParamsAt(old_s, params_ptr).num_aux != RecAuxCount(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxAligned16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AuxAlias16(old_s, rec_ptr, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AuxStateEqual16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rec_index == RealmAt(new_s, rd).rec_index + 1)
  && (result.is_Ok() ==> GranuleAt(new_s, rec_ptr).state == REC)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).owner == rd)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).attest_state == NO_ATTEST_IN_PROGRESS)
  && (result.is_Ok() ==> MpidrEqual(RecAt(new_s, rec_ptr).mpidr, RmiRecParamsAt(new_s, params_ptr).mpidr))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).state == REC_READY)
  && (result.is_Ok() && RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_RUNNABLE ==> RecAt(new_s, rec_ptr).flags.runnable == RUNNABLE)
  && (result.is_Ok() && RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_NOT_RUNNABLE ==> RecAt(new_s, rec_ptr).flags.runnable == NOT_RUNNABLE)
  && (result.is_Ok() ==> (RecAt(new_s, rec_ptr).gprs[0] == RmiRecParamsAt(new_s, params_ptr).gprs[0] && RecAt(new_s, rec_ptr).gprs[1] == RmiRecParamsAt(new_s, params_ptr).gprs[1] && RecAt(new_s, rec_ptr).gprs[2] == RmiRecParamsAt(new_s, params_ptr).gprs[2] && RecAt(new_s, rec_ptr).gprs[3] == RmiRecParamsAt(new_s, params_ptr).gprs[3] && RecAt(new_s, rec_ptr).gprs[4] == RmiRecParamsAt(new_s, params_ptr).gprs[4] && RecAt(new_s, rec_ptr).gprs[5] == RmiRecParamsAt(new_s, params_ptr).gprs[5] && RecAt(new_s, rec_ptr).gprs[6] == RmiRecParamsAt(new_s, params_ptr).gprs[6] && RecAt(new_s, rec_ptr).gprs[7] == RmiRecParamsAt(new_s, params_ptr).gprs[7] && RecAt(new_s, rec_ptr).gprs[8] == 0 && RecAt(new_s, rec_ptr).gprs[9] == 0 && RecAt(new_s, rec_ptr).gprs[10] == 0 && RecAt(new_s, rec_ptr).gprs[11] == 0 && RecAt(new_s, rec_ptr).gprs[12] == 0 && RecAt(new_s, rec_ptr).gprs[13] == 0 && RecAt(new_s, rec_ptr).gprs[14] == 0 && RecAt(new_s, rec_ptr).gprs[15] == 0 && RecAt(new_s, rec_ptr).gprs[16] == 0 && RecAt(new_s, rec_ptr).gprs[17] == 0 && RecAt(new_s, rec_ptr).gprs[18] == 0 && RecAt(new_s, rec_ptr).gprs[19] == 0 && RecAt(new_s, rec_ptr).gprs[20] == 0 && RecAt(new_s, rec_ptr).gprs[21] == 0 && RecAt(new_s, rec_ptr).gprs[22] == 0 && RecAt(new_s, rec_ptr).gprs[23] == 0 && RecAt(new_s, rec_ptr).gprs[24] == 0 && RecAt(new_s, rec_ptr).gprs[25] == 0 && RecAt(new_s, rec_ptr).gprs[26] == 0 && RecAt(new_s, rec_ptr).gprs[27] == 0 && RecAt(new_s, rec_ptr).gprs[28] == 0 && RecAt(new_s, rec_ptr).gprs[29] == 0 && RecAt(new_s, rec_ptr).gprs[30] == 0 && RecAt(new_s, rec_ptr).gprs[31] == 0))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).pc == RmiRecParamsAt(new_s, params_ptr).pc)
  && (result.is_Ok() && RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_RUNNABLE ==> RealmAt(new_s, rd).measurements[0] == RimExtendRec(new_s, RealmAt(new_s, rd), RmiRecParamsAt(new_s, params_ptr)))
  && (result.is_Ok() ==> AuxEqual16(new_s, RecAt(new_s, rec_ptr).aux, RmiRecParamsAt(new_s, params_ptr).aux, RecAuxCount(new_s, rd)))
  && (result.is_Ok() ==> AuxStateEqual16(new_s, RecAt(new_s, rec_ptr).aux, RecAuxCount(new_s, rd) as int, REC_AUX))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).ripas_addr == 0)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).ripas_top == 0)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).pending == REC_PENDING_NONE)
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_recs == RealmAt(new_s, rd).num_recs + 1)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).gic_owner == 0)
  && ((AddrIsGranuleAligned(old_s, params_ptr) &&
       GranuleAccessPermitted(old_s, params_ptr, PAS_NS) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegableDram(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       !(RealmAt(old_s, rd).num_recs == pow2(ImplFeatures(old_s).max_recs_order as nat) - 1) &&
       !(RecIndex(old_s, RmiRecParamsAt(old_s, params_ptr).mpidr) != RealmAt(old_s, rd).rec_index) &&
       !(RmiRecParamsAt(old_s, params_ptr).num_aux != RecAuxCount(old_s, rd)) &&
       AuxAligned16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int) &&
       !(AuxAlias16(old_s, rec_ptr, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int)) &&
       AuxStateEqual16(old_s, RmiRecParamsAt(old_s, params_ptr).aux, RmiRecParamsAt(old_s, params_ptr).num_aux as int, DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rec_index == RealmAt(old_s, rd).rec_index)
  && (result.is_Err()
    ==> GranuleAt(new_s, rec_ptr).state == GranuleAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).owner == RecAt(old_s, rec_ptr).owner)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).attest_state == RecAt(old_s, rec_ptr).attest_state)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).state == RecAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).flags.runnable == RecAt(old_s, rec_ptr).flags.runnable)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).flags.runnable == RecAt(old_s, rec_ptr).flags.runnable)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).pc == RecAt(old_s, rec_ptr).pc)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_addr == RecAt(old_s, rec_ptr).ripas_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).ripas_top == RecAt(old_s, rec_ptr).ripas_top)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).pending == RecAt(old_s, rec_ptr).pending)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_recs == RealmAt(old_s, rd).num_recs)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).gic_owner == RecAt(old_s, rec_ptr).gic_owner)
  && (!(result.is_Ok() && (RmiRecParamsAt(old_s, params_ptr).flags.runnable == RMI_RUNNABLE)) ==> RecAt(new_s, rec_ptr).flags.runnable == RecAt(old_s, rec_ptr).flags.runnable)
}

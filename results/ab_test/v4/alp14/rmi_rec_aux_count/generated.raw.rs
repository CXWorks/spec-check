pub open spec fn RMI_REC_AUX_COUNT_spec(old_s: S, new_s: S, rd: Address, result: Result<(), RmiStatusCode>, aux_count: u64) -> bool {
    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (result.is_Ok() ==> aux_count == RecAuxCount(old_s, rd)) &&
     (result.is_Ok() ==> new_s == old_s))
}
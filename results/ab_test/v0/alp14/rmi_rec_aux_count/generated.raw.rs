```verus
pub open spec fn RMI_REC_AUX_COUNT_spec(s: S, rd: Address, result: Result<(), RmiStatusCode>, aux_count: u64) -> bool {
    (
        !AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        !PaIsDelegable(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        GranuleAt(s, rd).state != RmmGranuleState::RD ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        (
            AddrIsGranuleAligned(s, rd) && 
            PaIsDelegable(s, rd) && 
            GranuleAt(s, rd).state == RmmGranuleState::RD
        ) ==> (
            result.is_Ok() && aux_count == RecAuxCount(s, rd)
        )
    )
}
```
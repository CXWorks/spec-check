pub open spec fn ffa_el3_intr_handle_spec(result: Result<(), FfaErrorCode>, old_s: S, new_s: S) -> bool {
    (!FfaInstanceIsSecurePhysicalSmc(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((CallerIsSEl1OrSEl2(old_s) && ScrEl3Fiq(old_s) == 1) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((FfaInstanceIsSecurePhysicalSmc(old_s)
            && !(CallerIsSEl1OrSEl2(old_s) && ScrEl3Fiq(old_s) == 1))
        ==> (result.is_Ok()
            && SecurityState(new_s) == SecurityState(old_s)
            && CallerExceptionLevel(new_s) == CallerExceptionLevel(old_s)))
    && (result.is_Err() ==> (SecurityState(new_s) == SecurityState(old_s)
            && CallerExceptionLevel(new_s) == CallerExceptionLevel(old_s)))
}

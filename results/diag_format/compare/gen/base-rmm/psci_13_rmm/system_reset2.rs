pub open spec fn system_reset2_spec(result: PsciReturnCode, reset_type: UInt32, cookie: UInt64, old_s: S, new_s: S) -> bool {
    (!IsImplemented(SYSTEM_RESET2) ==> ResultEqual(result, NOT_SUPPORTED))
    && (reset_type[31] == 0 && reset_type[30:0] != SYSTEM_WARM_RESET ==> ResultEqual(result, INVALID_PARAMETERS))
    && (reset_type == SYSTEM_WARM_RESET ==> (MainMemoryPreserved() || FellBackToColdReset()))
    && (reset_type == SYSTEM_WARM_RESET ==> AllMemoryRequestersReset())
    && (reset_type == SYSTEM_WARM_RESET ==> AllCpusAndMmusReset())
    && (reset_type == SYSTEM_WARM_RESET ==> AllInterruptsDisabled())
    && (reset_type == SYSTEM_WARM_RESET ==> SmmuStateEqualsColdResetState())
    && (reset_type == SYSTEM_WARM_RESET ==> CookieIgnored(cookie))
}
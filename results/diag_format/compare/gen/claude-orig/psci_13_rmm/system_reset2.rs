pub open spec fn system_reset2_spec(fid: u32, reset_type: u32, cookie: u64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsImplemented(fid) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((IsImplemented(fid) && (reset_type & 0x8000_0000u32) == 0 && (reset_type & 0x7FFF_FFFFu32) != 0) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((IsImplemented(fid) && reset_type == 0) ==> (
        (MainMemoryPreserved() || FellBackToColdReset())
        && AllMemoryRequestersReset()
        && AllCpusAndMmusReset()
        && AllInterruptsDisabled()
        && SmmuStateEqualsColdResetState()
        && CookieIgnored(cookie)
    ))
}

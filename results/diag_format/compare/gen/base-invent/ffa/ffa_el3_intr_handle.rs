pub open spec fn ffa_el3_intr_handle_spec(result: FfaReturnCode, old_s: S, new_s: S) -> bool {
    (FFAInstanceIsUnsupported(old_s) ==> result == FFA_ERROR_NOT_SUPPORTED)
    && (ScrEl3FiqIsSet(old_s) ==> result == FFA_ERROR_NOT_SUPPORTED)
    && (result == FFA_SUCCESS ==> true)
}
pub open spec fn ffa_spm_id_get_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (new_s.spm_id == old_s.spm_id))
    && (result != RSI_SUCCESS ==> true)
}
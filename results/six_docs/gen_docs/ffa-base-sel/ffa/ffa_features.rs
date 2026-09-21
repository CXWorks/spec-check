pub open spec fn ffa_features_spec(result: int, old_s: S, new_s: S) -> bool {
    let function_id: int = old_s.cmd_input_0;
    let ff_a_fid_or_feature: int = old_s.cmd_input_1;
    let input_properties: int = old_s.cmd_input_2;
    let is_ffa_fid: bool = (ff_a_fid_or_feature & 0x80000000) != 0;
    let ff_a_fid: int = if is_ffa_fid { ff_a_fid_or_feature & 0x7FFFFFFF } else { 0 };
    let feature_id: int = if !is_ffa_fid { ff_a_fid_or_feature & 0xFF } else { 0 };
    let is_not_supported: bool = result == FFA_NOT_SUPPORTED;
    let is_success: bool = result == FFA_SUCCESS;
    (is_not_supported ==> (is_ffa_fid ==> (function_id == 0x84000064) && (feature_id == 0)))
    && (is_not_supported ==> (!is_ffa_fid ==> (function_id == 0x84000064) && (feature_id != 0)))
    && (is_success ==> (function_id == 0x84000064))
    && (is_success ==> (is_ffa_fid ==> (input_properties == 0)))
    && (is_success ==> (!is_ffa_fid ==> (input_properties == 0)))
    && (is_success ==> (new_s == old_s))
}
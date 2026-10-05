pub open spec fn ffa_id_get_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.w1 != 0 || old_s.w2 != 0 || old_s.w3 != 0 || old_s.w4 != 0 || old_s.w5 != 0 || old_s.w6 != 0 || old_s.w7 != 0))
    && (result == RSI_SUCCESS ==> (new_s.w2 as int >= 0 && new_s.w2 as int <= 0xFFFF))
    && (result == RSI_SUCCESS ==> (new_s.w2 as int == 0 ==> old_s.is_physical_ffa_instance))
    && (result == RSI_SUCCESS ==> (new_s.w2 as int != 0 ==> !old_s.is_physical_ffa_instance))
    && (result == RSI_SUCCESS ==> (new_s.w2 as int != 0 ==> (new_s.w2 as int >= 1 && new_s.w2 as int <= 0xFFFF)))
    && (result == RSI_SUCCESS ==> (new_s.w3 == 0 && new_s.w4 == 0 && new_s.w5 == 0 && new_s.w6 == 0 && new_s.w7 == 0))
    && (result == RSI_SUCCESS ==> (new_s.w2 as int == 0 ==> old_s.is_physical_ffa_instance))
    && (result == RSI_SUCCESS ==> (new_s.w2 as int != 0 ==> !old_s.is_physical_ffa_instance))
    && (result == RSI_SUCCESS ==> (new_s.w2 as int != 0 ==> (new_s.w2 as int >= 1 && new_s.w2 as int <= 0xFFFF)))
    && (result == RSI_SUCCESS ==> (new_s.w3 == 0 && new_s.w4 == 0 && new_s.w5 == 0 && new_s.w6 == 0 && new_s.w7 == 0))
}
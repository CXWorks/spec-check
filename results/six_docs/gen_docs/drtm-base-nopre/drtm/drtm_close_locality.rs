pub open spec fn drtm_close_locality_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (old_s.drtm_locality_2_relinquished == false ==> result == RSI_ERROR_INPUT)
    && (old_s.drtm_locality_3_relinquished == false ==> result == RSI_ERROR_INPUT)
    && (old_s.drtm_locality_2_closed == true && old_s.drtm_locality_2 == 2 ==> result == RSI_ERROR_INPUT)
    && (old_s.drtm_locality_3_closed == true && old_s.drtm_locality_3 == 3 ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (new_s.drtm_locality_2_closed == old_s.drtm_locality_2_closed) && (new_s.drtm_locality_3_closed == old_s.drtm_locality_3_closed))
}
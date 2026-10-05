pub open spec fn drtm_close_locality_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (old_s.drtm_locality_relinquished(2) ==> result == RSI_SUCCESS)
    && (old_s.drtm_locality_relinquished(3) ==> result == RSI_SUCCESS)
    && (!old_s.drtm_locality_relinquished(2) && !old_s.drtm_locality_relinquished(3) ==> result == RSI_ERROR_INVALID_PARAMETERS)
    && (old_s.drtm_locality_relinquished(2) && !old_s.drtm_locality_relinquished(3) && result == RSI_SUCCESS ==> new_s.drtm_locality_relinquished(2) && !new_s.drtm_locality_relinquished(3))
    && (old_s.drtm_locality_relinquished(3) && !old_s.drtm_locality_relinquished(2) && result == RSI_SUCCESS ==> !new_s.drtm_locality_relinquished(2) && new_s.drtm_locality_relinquished(3))
    && (old_s.drtm_locality_relinquished(2) && old_s.drtm_locality_relinquished(3) && result == RSI_SUCCESS ==> new_s.drtm_locality_relinquished(2) && new_s.drtm_locality_relinquished(3))
    && (result == RSI_ERROR_NOT_SUPPORTED ==> old_s.drtm_supported == false)
    && (result == RSI_ERROR_DENIED ==> old_s.drtm_denied == true)
    && (result == RSI_ERROR_INVALID_PARAMETERS ==> old_s.drtm_locality_relinquished(2) == false && old_s.drtm_locality_relinquished(3) == false)
    && (result == RSI_ERROR_ALREADY_CLOSED ==> old_s.drtm_locality_relinquished(2) == true || old_s.drtm_locality_relinquished(3) == true)
}
pub open spec fn telemetry_list_shmti_spec(result: RsiCommandReturnCode, num_SHMTI: uint32, SHMTI_desc: [uint32; 5], old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (num_SHMTI == 0 || (SHMTI_desc.len() as int != 5)))
    && (result == RSI_ERROR_STATE ==> (num_SHMTI == 0 || (SHMTI_desc.len() as int != 5)))
    && (result == RSI_INCOMPLETE ==> (num_SHMTI == 0 || (SHMTI_desc.len() as int != 5)))
    && (result == RSI_ERROR_UNKNOWN ==> (num_SHMTI == 0 || (SHMTI_desc.len() as int != 5)))
    && (result == RSI_SUCCESS ==> (num_SHMTI > 0 && SHMTI_desc.len() as int == 5 && SHMTI_desc[4] == 0))
    && (result != RSI_SUCCESS ==> (num_SHMTI == 0 || (SHMTI_desc.len() as int != 5)))
}
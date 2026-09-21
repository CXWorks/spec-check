pub open spec fn sdei_version_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_NOT_SUPPORTED ==> (old_s.sdei_supported == false))
    && (result == RSI_SUCCESS ==> (old_s.sdei_supported == true))
    && (result == RSI_SUCCESS ==> (new_s.sdei_supported == old_s.sdei_supported))
    && (result == RSI_SUCCESS ==> (new_s.sdei_version == old_s.sdei_version))
    && (result == RSI_SUCCESS ==> (new_s.sdei_version as int >= 0))
    && (result == RSI_SUCCESS ==> ((new_s.sdei_version as int) & 0x1_FFFF_FFFF == 0))
    && (result == RSI_SUCCESS ==> ((new_s.sdei_version as int) >> 48 == 1))
    && (result == RSI_SUCCESS ==> ((new_s.sdei_version as int) >> 32 == 1))
}
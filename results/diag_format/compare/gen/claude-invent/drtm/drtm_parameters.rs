pub open spec fn drtm_parameters_spec(
    s: S,
    params_addr: UInt64,
    revision: u16,
    reserved: u16,
    launch_features: UInt32,
    dlme_region_addr: UInt64,
    dlme_region_size: UInt64,
    dlme_image_start: UInt64,
    dlme_entry_point_offset: UInt64,
    dlme_image_size: UInt64,
    dlme_data_offset: UInt64,
    nw_dce_region_addr: UInt64,
    nw_dce_region_size: UInt64,
    mpt_addr: UInt64,
    mpt_size: UInt64,
) -> bool {
    (revision == 2u16)
    && (reserved == 0u16)
    && ((launch_features >> 8u32) == 0u32)
    && (((launch_features >> 3u32) & 7u32) <= 1u32)
    && (((launch_features >> 1u32) & 3u32) <= 1u32)
    && ((((launch_features >> 1u32) & 3u32) == 1u32) ==> (((launch_features >> 6u32) & 1u32) == 1u32))
    && IsNonSecureContiguous(s, params_addr as int, 88int)
    && ((params_addr as int) % 4096 == 0)
    && (((dlme_region_addr as int) + (dlme_region_size as int)) <= 0x1_0000_0000_0000_0000int)
    && (((nw_dce_region_addr as int) + (nw_dce_region_size as int)) <= 0x1_0000_0000_0000_0000int)
    && (((mpt_addr as int) + (mpt_size as int)) <= 0x1_0000_0000_0000_0000int)
    && !(((dlme_region_addr as int) < (nw_dce_region_addr as int) + (nw_dce_region_size as int))
        && ((nw_dce_region_addr as int) < (dlme_region_addr as int) + (dlme_region_size as int)))
    && !(((dlme_region_addr as int) < (mpt_addr as int) + (mpt_size as int))
        && ((mpt_addr as int) < (dlme_region_addr as int) + (dlme_region_size as int)))
    && !(((nw_dce_region_addr as int) < (mpt_addr as int) + (mpt_size as int))
        && ((mpt_addr as int) < (nw_dce_region_addr as int) + (nw_dce_region_size as int)))
    && (((dlme_image_start as int) + (dlme_image_size as int)) <= (dlme_region_size as int))
    && ((dlme_entry_point_offset as int) < (dlme_image_size as int))
    && DlmeRegionMeetsRequirements(s, dlme_region_addr as int, dlme_region_size as int, dlme_image_start as int, dlme_image_size as int, dlme_data_offset as int)
    && (((nw_dce_region_addr == 0u64) && (nw_dce_region_size == 0u64))
        || (((nw_dce_region_addr as int) % 4096 == 0)
            && IsNonSecureContiguous(s, nw_dce_region_addr as int, nw_dce_region_size as int)))
    && ((((launch_features >> 3u32) & 7u32) != 1u32) ==> ((mpt_addr == 0u64) && (mpt_size == 0u64)))
    && ((mpt_addr as int) % 4096 == 0)
}

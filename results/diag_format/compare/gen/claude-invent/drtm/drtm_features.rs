pub open spec fn drtm_features_spec(x1: u64, x0: i64, x1_out: u64, old_s: S, new_s: S) -> bool {
    let is_feature_query = (x1 >> 63u64) == 1u64;
    let function_id = (x1 & 0xFFFF_FFFFu64) as u32;
    let feature_id = (x1 & 0xFFu64) as u8;
    let fn_reserved_zero = ((x1 >> 32u64) & 0x7FFF_FFFFu64) == 0u64;
    let feat_reserved_zero = ((x1 >> 8u64) & 0x7F_FFFF_FFFF_FFFFu64) == 0u64;
    let fn_query_ok = !is_feature_query && fn_reserved_zero;
    let feat_query_ok = is_feature_query && feat_reserved_zero;
    (new_s == old_s)
    && ((fn_query_ok && !DrtmFunctionIdImplemented(old_s, function_id)) ==> x0 == NOT_SUPPORTED)
    && ((feat_query_ok && !DrtmFeatureIdImplemented(old_s, feature_id)) ==> x0 == NOT_SUPPORTED)
    && ((fn_query_ok && DrtmFunctionIdImplemented(old_s, function_id)) ==> x0 >= 0)
    && ((feat_query_ok && DrtmFeatureIdImplemented(old_s, feature_id)) ==> (
        x0 >= 0
        && ((x0 > 0 && feature_id == 0x1u8) ==> (x1_out >> 37u64) == 0u64)
        && ((x0 > 0 && feature_id == 0x2u8 && !DrtmUsesNormalWorldDce(old_s)) ==> (x1_out >> 32u64) == 0u64)
        && ((x0 > 0 && feature_id == 0x3u8) ==> (
            (x1_out >> 24u64) == 0u64
            && (((x1_out & 0x2u64) != 0u64) ==> ((x1_out >> 8u64) & 0xFFFFu64) != 0u64)
        ))
        && ((x0 > 0 && feature_id == 0x4u8) ==> x1_out == DrtmBootPeId(old_s))
        && ((x0 > 0 && feature_id == 0x5u8) ==> (x1_out >> 8u64) == 0u64)
        && ((x0 > 0 && feature_id == 0x6u8) ==> (x1_out >> 1u64) == 0u64)
    ))
}

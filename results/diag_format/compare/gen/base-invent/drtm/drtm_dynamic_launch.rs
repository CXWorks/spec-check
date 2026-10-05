pub open spec fn drtm_dynamic_launch_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!R65000(old_s) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!R65010(old_s) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (R65000(old_s) && R65010(old_s) ==> result.is_Ok())
}
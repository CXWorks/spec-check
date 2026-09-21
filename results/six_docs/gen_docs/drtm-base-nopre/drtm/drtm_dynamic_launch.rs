pub open spec fn drtm_dynamic_launch_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (RmiStatusCode::RMI_ERROR_INPUT == RmiStatusCode::RMI_ERROR_INPUT ==> result.is_Err())
    && (result.is_Ok() ==> true)
}
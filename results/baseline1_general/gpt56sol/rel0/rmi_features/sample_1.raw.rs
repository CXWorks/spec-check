pub open spec fn rmi_features_spec(result: Result<(), RmiStatusCode>, fid: UInt64, index: UInt64, value: Bits64, old_s: S, new_s: S) -> bool {
    result.is_Ok()
    && (index != 0 ==> value == 0)
    && old_s == new_s
}
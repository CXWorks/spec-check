pub open spec fn rmi_features_spec(result: Result<(), RmiStatusCode>, index: UInt64, value: Bits64, old_s: S, new_s: S) -> bool {
    (index != 0 ==> result.is_Ok() && value == 0)
    && (new_s == old_s)
}
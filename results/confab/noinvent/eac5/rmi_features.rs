pub open spec fn rmi_features_spec(result: Result<(), RmiStatusCode>, index: u64, value: u64, old_s: S, new_s: S) -> bool {
    (index != 0 ==> value == 0)
}
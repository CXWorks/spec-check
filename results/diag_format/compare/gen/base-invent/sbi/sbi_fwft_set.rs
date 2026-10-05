pub open spec fn sbi_fwft_set_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.code == 0 ==> (new_s.firmware_features == old_s.firmware_features))
    && (result.code != 0 ==> (result.code == -1))
}
pub open spec fn sbi_nacl_sync_csr_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == 0)
}
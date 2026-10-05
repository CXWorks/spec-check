pub open spec fn sbi_nacl_sync_csr_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (!((result.error as int) == 0) ==> (result.error as int) == SBI_NACL_ERR_INVALID_CSR)
    && (result.error as int) == 0
}
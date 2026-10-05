use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub csr_value: u64,
}

pub spec const SBI_NACL_ERR_INVALID_CSR: int = -3;

} // verus!

pub open spec fn ffa_partition_info_get_regs_spec(result: u32, error_code: i32, last_index: u16, current_index: u16, info_tag: u16, desc_size: u16, partition_info: [u64; 15], old_s: S, new_s: S) -> bool {
    (!IsValidUuid(old_s, uuid_lo(old_s), uuid_hi(old_s)) ==> ResultEqual(error_code, INVALID_PARAMETERS))
    && (!IsValidStartIndex(old_s, uuid_lo(old_s), uuid_hi(old_s), start_index(old_s)) ==> ResultEqual(error_code, INVALID_PARAMETERS))
    && (!IsImplementedAtInstance(old_s, FFA_PARTITION_INFO_GET_REGS, ffa_instance(old_s)) ==> ResultEqual(error_code, NOT_SUPPORTED))
    && (!CalleeInStateToHandleRequest(old_s) ==> ResultEqual(error_code, DENIED))
    && (start_index(old_s) > 0 && tag(old_s) != CalleeInfoTag(old_s, uuid_lo(old_s), uuid_hi(old_s)) ==> ResultEqual(error_code, RETRY))
    && (!CalleeReadyToHandleRequest(old_s) ==> ResultEqual(error_code, NOT_READY))
    && ResultEqual(result, FFA_SUCCESS64)
    && (last_index(old_s) + 1 == PartitionInfoCount(old_s, uuid_lo(old_s), uuid_hi(old_s)))
    && (last_index(old_s) >= start_index(old_s))
    && (current_index(old_s) >= start_index(old_s))
    && (current_index(old_s) <= last_index(old_s))
    && (info_tag(old_s) == CalleeInfoTag(old_s, uuid_lo(old_s), uuid_hi(old_s)))
    && (desc_size(old_s) == 48)
    && (forall|i: u64| (start_index(old_s) <= i && i <= current_index(old_s)) ==> DescriptorAt(partition_info, i - start_index(old_s)) == PartitionInfoEntry(old_s, uuid_lo(old_s), uuid_hi(old_s), i))
    && (!IsNilUuid(old_s, uuid_lo(old_s), uuid_hi(old_s)) ==> (forall|n: u64| IsReturnedDescriptorBase(n, start_index(old_s), current_index(old_s)) ==> (Reg(n + 1) == 0 && Reg(n + 2) == 0)))
    && (forall|n: u64| (3 <= n && n <= 17 && !IsReturnedDescriptorRegister(n, start_index(old_s), current_index(old_s))) ==> Reg(n) == 0)
    && (old_s == new_s)
}

fn uuid_lo(s: S) -> u64 { s.x1[0..8] }
fn uuid_hi(s: S) -> u64 { s.x2[0..8] }
fn start_index(s: S) -> u16 { s.x3[0..2] }
fn tag(s: S) -> u16 { s.x3[2..4] }
fn ffa_instance(s: S) -> u32 { s.x0[0..4] }
fn CalleeInfoTag(s: S, uuid_lo: u64, uuid_hi: u64) -> u16 { /* implementation dependent */ }
fn IsValidUuid(s: S, uuid_lo: u64, uuid_hi: u64) -> bool { /* implementation dependent */ }
fn IsValidStartIndex(s: S, uuid_lo: u64, uuid_hi: u64, start_index: u16) -> bool { /* implementation dependent */ }
fn IsImplementedAtInstance(s: S, fid: u32, instance: u32) -> bool { /* implementation dependent */ }
fn CalleeInStateToHandleRequest(s: S) -> bool { /* implementation dependent */ }
fn CalleeReadyToHandleRequest(s: S) -> bool { /* implementation dependent */ }
fn PartitionInfoCount(s: S, uuid_lo: u64, uuid_hi: u64) -> u16 { /* implementation dependent */ }
fn DescriptorAt(partition_info: [u64; 15], i: u64) -> u64 { partition_info[i as usize] }
fn PartitionInfoEntry(s: S, uuid_lo: u64, uuid_hi: u64, i: u64) -> u64 { /* implementation dependent */ }
fn IsNilUuid(s: S, uuid_lo: u64, uuid_hi: u64) -> bool { uuid_lo == 0 && uuid_hi == 0 }
fn IsReturnedDescriptorBase(n: u64, start_index: u16, current_index: u16) -> bool { /* implementation dependent */ }
fn Reg(n: u64) -> u64 { /* implementation dependent */ }
fn ResultEqual(a: u32, b: u32) -> bool { a == b }
fn INVALID_PARAMETERS: u32 = 1
fn NOT_SUPPORTED: u32 = 2
fn DENIED: u32 = 3
fn RETRY: u32 = 4
fn NOT_READY: u32 = 5
fn FFA_SUCCESS64: u32 = 0
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid, IsValidStartIndex, IsImplementedAtInstance, CalleeInStateToHandleRequest, CalleeReadyToHandleRequest, PartitionInfoCount, DescriptorAt, PartitionInfoEntry, IsNilUuid, IsReturnedDescriptorBase, Reg, ResultEqual, INVALID_PARAMETERS, NOT_SUPPORTED, DENIED, RETRY, NOT_READY, FFA_SUCCESS64: Symbol = /* symbols */
fn old_s, new_s: Parameter = /* parameters */
fn u32, u64, u16, i32: Type = /* types */
fn [u64; 15]: Type = /* array type */
fn S: Type = /* state type */
fn x0, x1, x2, x3: Field = /* register fields */
fn uuid_lo, uuid_hi, start_index, tag, ffa_instance, CalleeInfoTag, IsValidUuid
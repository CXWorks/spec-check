pub open spec fn base_notify_errors__3_2_2_10_spec(notify_enable: UInt32, agent_id: UInt32, status: i32, old_s: S, new_s: S) -> bool {
    (((notify_enable & 0xFFFF_FFFEu32) != 0u32) ==> status == INVALID_PARAMETERS)
    && (status == SUCCESS ==> (
        ((notify_enable & 0xFFFF_FFFEu32) == 0u32)
        && (ErrorNotifyEnabled(new_s, agent_id) == ((notify_enable & 1u32) == 1u32))
        && (forall|a: UInt32| a != agent_id ==> ErrorNotifyEnabled(new_s, a) == ErrorNotifyEnabled(old_s, a))
    ))
    && (status != SUCCESS ==> new_s == old_s)
}

pub open spec fn base_discover_agent__3_2_2_9_spec(agent_id: u32, status: i32, out_agent_id: u32, name: Seq<u8>, old_s: S, new_s: S) -> bool {
    (!AgentIdIsValid(old_s, agent_id) ==> status == NOT_FOUND)
    && (AgentIdIsValid(old_s, agent_id) ==> (
        status == SUCCESS
        && (agent_id == 0xFFFF_FFFFu32 ==> out_agent_id == CallingAgentId(old_s))
        && (agent_id != 0xFFFF_FFFFu32 ==> out_agent_id == agent_id)
        && name.len() == 16
        && (exists|i: int| 0 <= i < 16 && name[i] == 0u8)
        && name =~= AgentName(old_s, out_agent_id)
        && (out_agent_id == 0u32 ==> name.subrange(0, 8) =~= seq![0x70u8, 0x6cu8, 0x61u8, 0x74u8, 0x66u8, 0x6fu8, 0x72u8, 0x6du8])
    ))
    && new_s == old_s
}

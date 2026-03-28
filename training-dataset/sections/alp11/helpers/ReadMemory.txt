B3.60 ReadMemory function
             Read contents of memory at address range [addr + offset, addr + offset + size)
             offset and size are both numbers of bytes.
             func ReadMemory(
                 addr : bits(64),
                 offset : integer,
                 size : integer) => bits(size * 8)

pub struct VirtualMachine {
    memory_ptr: u32,
    memory: [u8; 100]
}

impl VirtualMachine {

    pub fn new() -> VirtualMachine {
        VirtualMachine { memory_ptr: 0, memory: [0; 100] }
    }
    
    pub fn memory_ptr(&self) -> &u32 {
        &self.memory_ptr
    }
}

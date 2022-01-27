#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::io::{Read, Write};
use std::mem::size_of;


#[derive(Debug)]
pub enum ArcVMError {
    /*
        * STACK
            * PUSH
                * OVRFLW
            * POP
                * UDRFLW
            * MATH
                * DIVZERO
            * LOGIC
            * INSTR
            * OVRFLW
            * OTHER
                * FAILED
        * MEMORY
            * OVRFLW
            * ALLOC
                * FAILED
        * ADDREG
            * OVRFLW
        * RETSTACK
            * OVRFLW
        * CTR
            * OVRFLW
        * IO
            * INT
            * NOACCEPT
                * IN
                * OUT
    */

    STACK_PUSH_OVRFLW_ERROR,
    STACK_POP_UDRFLW_ERROR,
    STACK_MATH_DIVZERO_ERROR,
    STACK_OTHER_FAILED_ERROR,

    MEMORY_ALLOC_FAILED_ERROR,

    IO_NOACCEPT_IN_ERROR,
    IO_NOACCEPT_OUT_ERROR,
}

pub type Operand = u32;             // Defines the Basic piece of the data stack
                                    // Search for FIXED_SIZE in this file for safety
pub type Counter = u32;

pub fn operand_size() -> usize {
    size_of::<Operand>()
}

trait ArcData {
    fn as_bool(&self) -> bool;
}

impl ArcData for Operand {
    fn as_bool(&self) -> bool {
        if self == &0x00 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
pub struct Memory {                     // Defines memory the size of RAM
    pub address_register: usize,
    pub heap: Vec<u8>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            address_register: 0,
            heap: Vec::with_capacity(1024),
        }
    }

    pub fn operand_to_address(addr: Operand) -> usize {
        let addr_bytes: [u8; 4] = addr.to_be_bytes(); // FIXED_SIZE
        let mut addr_full_bytes: [u8; 8] = [0x00; 8];

        let mut i: usize = 0;

        while i < operand_size() {
            addr_full_bytes[i + operand_size()] = addr_bytes[i];
            i += 1;
        }

        usize::from_be_bytes(addr_full_bytes)
    }

    pub fn operand_to_char(chr: Operand) -> u8 {
        let chr_bytes: [u8; 4] = chr.to_be_bytes(); // FIXED_SIZE
        chr_bytes[3]
    }

    fn operand_to_ioperand(o: Operand) -> i32 { // FIXED_SIZE
        i32::from_be_bytes(o.to_be_bytes())
    }

    fn ioperand_to_operand(i: i32) -> Operand { // FIXED_SIZE
        Operand::from_be_bytes(i.to_be_bytes())
    }

    fn bool_to_operand(b: bool) -> Operand {
        if b {
            0x01
        } else {
            0x00
        }
    }

    pub fn malloc(&mut self, addr: usize, bytes_to_allocate: usize) -> bool {
        // ** Memory allocation **
        // Resize the vec to the right size if need be
        // Get a slice of the vector and return if it is empty or not
        // Note: expect big-endianness

        self.heap.reserve_exact(addr + bytes_to_allocate - 1);
        
        match self.heap.get(addr..(addr + bytes_to_allocate - 1)) {
            Some(_) => {  false },
            None => {
                let mut i = 0;
                while i < (addr + bytes_to_allocate) {
                    self.heap.push(0x00);
                    i += 1;
                }
                true
            },
        }
    }

    pub fn mstore(&mut self, addr: usize, data: Operand) {
        if self.heap.len() < operand_size() {
            return (); // Don't do anything if it will panic anyway
        }

        let mut i: usize = 0;

        while i < operand_size() {
            self.heap[addr + i] = data.to_be_bytes()[i];
            i += 1;
        }
    }

    fn mload(&self, addr: usize) -> Operand {
        let mut buffer: [u8; 4] = [0x00; 4]; // FIXED_SIZE
        let mut i: usize = 0;

        while i < operand_size() {
            buffer[i] = self.heap[addr + i];
            i += 1;
        }

        Operand::from_be_bytes(buffer)
    }

    fn eq(a: Operand, b: Operand) -> Operand {
        if a == b {
            0x01
        } else {
            0x00
        }
    }

    fn ne(a: Operand, b: Operand) -> Operand {
        if a == b {
            0x00
        } else {
            0x01
        }
    }

    fn and(a: Operand, b: Operand) -> Operand {
        if a.as_bool() && b.as_bool() {
            0x01
        } else {
            0x00
        }
    }

    fn or(a: Operand, b: Operand) -> Operand {
        if a.as_bool() || b.as_bool() {
            0x01
        } else {
            0x00
        }
    }

    fn ge(a: Operand, b: Operand) -> Operand {
        if a.as_bool() >= b.as_bool() {
            0x01
        } else {
            0x00
        }
    }

    fn le(a: Operand, b: Operand) -> Operand {
        if a.as_bool() <= b.as_bool() {
            0x01
        } else {
            0x00
        }
    }

    fn gt(a: Operand, b: Operand) -> Operand {
        if a.as_bool() > b.as_bool() {
            0x01
        } else {
            0x00
        }
    }

    fn lt(a: Operand, b: Operand) -> Operand {
        if a.as_bool() < b.as_bool() {
            0x01
        } else {
            0x00
        }
    }
}

#[derive(Debug)]
pub struct ArcVM_Core<T> {              // Contains the stacks
    pub data_stack: Vec<T>,
    pub return_stack: Vec<Counter>,
}

#[derive(Debug)]
pub struct ArcVM<T: Read, U: Write> {   // Defines a VM with stacks, 
    pub core: ArcVM_Core<Operand>,      // a program counter, memory, and IO
    pub program_counter: Counter,
    pub program_memory: Memory,
    pub arcin: T,
    pub arcout: U,
}

impl<T, U> ArcVM<T, U> where
    T: Read,
    U: Write,
{

    pub fn default(i: T, o: U) -> ArcVM<T, U> {
        ArcVM::<T, U> {
            core: ArcVM_Core::<Operand> {
                data_stack: Vec::with_capacity(64),
                return_stack: Vec::with_capacity(256),
            },
            program_counter: 0,
            program_memory: Memory::new(),
            arcin: i,
            arcout: o,
        }
    }

    //      -- Data Operations --

    // Push operand to stack CONFIRMED FUNCTIONING
    pub fn pushi(&mut self, data: Operand) -> Result<(), ArcVMError> {
        match self.core.data_stack.try_reserve(1) {
            Ok(o) => { 
                self.core.data_stack.push(data); 
                Ok(o) 
            },
            Err(_) => { Err(ArcVMError::STACK_PUSH_OVRFLW_ERROR) },
        }
    }

    // Push value from heap to stack CONFIRMED FUNCTIONING
    pub fn push(&mut self, addr: usize) -> Result<(), ArcVMError> {
        match self.core.data_stack.try_reserve(1) {
            Ok(o) => { 
                self.core.data_stack.push(
                    self.program_memory.mload(addr)
                ); 
                Ok(o) 
            },
            Err(_) => { Err(ArcVMError::STACK_PUSH_OVRFLW_ERROR) },
        }
    }

    // Pop value from stack into heap CONFIRMED FUNCTIONING
    pub fn pop(&mut self, addr: usize) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(d) => {
                self.program_memory.mstore(addr, d);
                Ok(()) 
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Store value from stack into heap CONFIRMED FUNCTIONING
    pub fn store(&mut self) -> Result<(), ArcVMError> {
        let top: Option<Operand> = self.core.data_stack.pop();
        let next: Option<Operand> = self.core.data_stack.pop();
        let addr: usize = match next {
            Some(d) => Memory::operand_to_address(d),
            None => 0,
        };
        match top {
            Some(d) => { 
                if self.program_memory.malloc(addr, operand_size()) {
                    self.program_memory.mstore(addr, d);
                    Ok(())
                } else {
                    Err(ArcVMError::MEMORY_ALLOC_FAILED_ERROR)
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Load from the heap into the stack CONFIRMED FUNCTIONING
    pub fn load(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(d) => {
                self.core.data_stack.push(
                    self.program_memory.mload(
                        Memory::operand_to_address(d)
                    )
                ); Ok(())
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    //      -- Utilities --

    // Jumps to an instruction CONFIRMED FUNCTIONING
    pub fn jump(&mut self, pc: Counter) -> Result<(), ArcVMError> {
        /*
        Note: Jump, JumpZero, and JumpNotZero should not pop off the stack
        */
        self.program_counter = pc;
        Ok(())
    }

    // Pushes the program counter to the stack CONFIRMED FUNCTIONING
    pub fn pushpc(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.try_reserve(1) {
            Ok(_) => { 
                self.core.data_stack.push(Operand::from_be_bytes(
                    self.program_counter.to_be_bytes()
                )); Ok(())
            },
            Err(_) => { Err(ArcVMError::STACK_PUSH_OVRFLW_ERROR) },
        }
    }

    // Pops the stack off into the program counter CONFIRMED FUNCTIONING
    pub fn poppc(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(d) => { 
                self.program_counter = Counter::from_be_bytes(
                    d.to_be_bytes()
                ); 
                Ok(())
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Duplicates top of stack
    pub fn dup(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(d) => {
                self.pushi(d).unwrap();
                self.pushi(d)
            },
            None => { Err(ArcVMError::STACK_OTHER_FAILED_ERROR) }
        }
    }

    // Drops top of stack
    pub fn drop(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(_) => Ok(()),
            None => Err(ArcVMError::STACK_OTHER_FAILED_ERROR),
        }
    }

    // Duplicates second-to-top of stack
    pub fn over(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => { 
                match self.core.data_stack.pop() {
                    Some(a) => {
                        self.pushi(a).unwrap();
                        self.pushi(b).unwrap();
                        self.pushi(a)
                    },
                    None => Err(ArcVMError::STACK_OTHER_FAILED_ERROR),
                }
            },
            None => Err(ArcVMError::STACK_OTHER_FAILED_ERROR),
        }
    }

    // Duplicates second-to-top of stack
    pub fn dnext(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => { 
                match self.core.data_stack.pop() {
                    Some(_) => {
                        self.pushi(b)
                    },
                    None => Err(ArcVMError::STACK_OTHER_FAILED_ERROR)
                }
            },
            None => Err(ArcVMError::STACK_OTHER_FAILED_ERROR),
        }
    }

    //      -- Arithmetic Operations --

    // Pops two numbers off the stack, and pushes their sum onto the stack CONFIRMED FUNCTIONING
    pub fn add(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a + b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes their difference onto the stack CONFIRMED FUNCTIONING
    pub fn sub(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a - b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes their product onto the stack CONFIRMED FUNCTIONING
    pub fn mul(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a * b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops a number off the stack and makes it negative
    pub fn neg(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(d) => {
                self.pushi(Memory::ioperand_to_operand(
                    Memory::operand_to_ioperand(d) * -1)
                )
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    //      -- Bitwise Operations --

    // Pops two numbers off the stack, and pushes the left-shift result CONFIRMED FUNCTIONING
    pub fn shl(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a << b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the right-shift result CONFIRMED FUNCTIONING
    pub fn shr(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a >> b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the bitwise AND result CONFIRMED FUNCTIONING
    pub fn bw_and(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a & b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the bitwise OR result CONFIRM FUNCTIONING
    pub fn bw_or(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a | b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the bitwise XOR result CONFIRM FUNCTIONING
    pub fn bw_xor(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(a ^ b) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops a number off the stack and NOTs it
    pub fn bw_not(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(d) => {
                self.pushi(!d)
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    //      -- Logic Operations --

    // Pops two numbers off the stack, and pushes the logical AND result CONFIRMED FUNCTIONING
    pub fn and(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::and(a, b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the logical OR result CONFIRMED FUNCTIONING
    pub fn or(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::or(a, b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the logical EQ result CONFIRMED FUNCTIONING
    pub fn eq(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::eq(a,b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the logical NOT EQ result CONFIRMED FUNCTIONING
    pub fn ne(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::ne(a,b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the logical GE result CONFIRMED FUNCTIONING
    pub fn ge(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::ge(a,b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the logical LE result CONFIRMED FUNCTIONING
    pub fn le(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::le(a,b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the logical GT result CONFIRMED FUNCTIONING
    pub fn gt(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::gt(a,b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops two numbers off the stack, and pushes the logical LT result CONFIRMED FUNCTIONING
    pub fn lt(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(b) => {
                match self.core.data_stack.pop() {
                    Some(a) => { self.pushi(Memory::lt(a,b)) },
                    None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
                }
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    // Pops a number off the stack and logically NOTs it
    pub fn not(&mut self) -> Result<(), ArcVMError> {
        match self.core.data_stack.pop() {
            Some(d) => {
                self.pushi(Memory::bool_to_operand(!(d.as_bool())))
            },
            None => { Err(ArcVMError::STACK_POP_UDRFLW_ERROR) },
        }
    }

    //      -- Input/Output --

    // Reads from arcin
    pub fn ain(&mut self) -> Result<(), ArcVMError> {
        let mut buffer: [u8; 1] = [0; 1];
        match self.arcin.read(&mut buffer) {
            Ok(_) => { 
                match self.pushi(buffer[0] as Operand) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(ArcVMError::STACK_PUSH_OVRFLW_ERROR),
                }
            },
            Err(_) => Err(ArcVMError::IO_NOACCEPT_IN_ERROR),
        }
    }

    // Pops off the stack and prints to arcout as an ASCII char
    pub fn aout(&mut self) -> Result<(), ArcVMError> {
        let mut buffer: [u8; 1] = [0; 1];
        let top = self.core.data_stack.pop();
        match top {
            Some(d) => {
                buffer[0] = Memory::operand_to_char(d);
                match self.arcout.write(&buffer) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(ArcVMError::IO_NOACCEPT_OUT_ERROR),
                }
            },
            None => Err(ArcVMError::STACK_POP_UDRFLW_ERROR),
        }
    }
}
pub mod instruction_types {

    #[derive(Debug, Default, Clone, Copy)]
    pub struct RType {
        pub op: u8,
        pub rs: u8,
        pub rt: u8,
        pub rd: u8,
        pub shamt: u8,
        pub funct: u8,
    }

    #[derive(Debug, Default, Clone, Copy)]
    pub struct IType {
        pub op: u8,
        pub rs: u8,
        pub rt: u8,
        pub immediate: u16,
    }

    #[derive(Debug, Default, Clone, Copy)]
    pub struct JType {
        pub op: u8,
        pub addr: u32,
    }

    #[derive(Debug)]
    pub enum Instruction {
        RType(RType),
        IType(IType),
        JType(JType),
    }

    impl Default for Instruction {
        fn default() -> Self {
            Instruction::RType(RType::default())
        }
    }
}
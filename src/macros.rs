/// Generates the following:
///
/// - A unique type for each opcode instruction type.
/// - Register and immediate value access methods for each opcode instruction type.
/// - An enum over all possible opcodes.
/// - An enum over all possible instructions.
macro_rules! impl_opcodes {
    // Recursively declares a unique struct for each opcode.
    (decl_op_struct $doc:literal $ix:literal $Op:ident $op:ident [$($field:ident)*] $($rest:tt)*) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $Op([u8; 3]);
        impl_opcodes!(decl_op_struct $($rest)*);
    };
    (decl_op_struct) => {};

    // Define the `OpcodeRepr` enum.
    (decl_opcode_enum $($doc:literal $ix:literal $Op:ident $op:ident [$($field:ident)*])*) => {
        /// Solely the opcode portion of an instruction represented as a single byte.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[repr(u8)]
        pub enum Opcode {
            $(
                #[doc = $doc]
                $Op = $ix,
            )*
        }
    };

    // Define the `Opcode` enum.
    (decl_instruction_enum $($doc:literal $ix:literal $Op:ident $op:ident [$($field:ident)*])*) => {
        /// Representation of a single instruction for the interpreter.
        ///
        /// The opcode is represented in the tag (variant), or may be retrieved in the form of an
        /// `Opcode` byte using the `opcode` method.
        ///
        /// The register and immediate data associated with the instruction is represented within
        /// an inner unit type wrapper around the 3 remaining bytes.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Instruction {
            $(
                #[doc = $doc]
                $Op($Op),
            )*
        }
    };

    // Generate a constructor based on the field layout.
    (impl_op_new [RegId]) => {
        /// Construct the instruction from its parts.
        pub fn new(ra: RegId) -> Self {
            Self(bytes_from_ra(ra))
        }
    };
    (impl_op_new [RegId RegId]) => {
        /// Construct the instruction from its parts.
        pub fn new(ra: RegId, rb: RegId) -> Self {
            Self(bytes_from_ra_rb(ra, rb))
        }
    };
    (impl_op_new [RegId RegId RegId]) => {
        /// Construct the instruction from its parts.
        pub fn new(ra: RegId, rb: RegId, rc: RegId) -> Self {
            Self(bytes_from_ra_rb_rc(ra, rb, rc))
        }
    };
    (impl_op_new [RegId RegId RegId RegId]) => {
        /// Construct the instruction from its parts.
        pub fn new(ra: RegId, rb: RegId, rc: RegId, rd: RegId) -> Self {
            Self(bytes_from_ra_rb_rc_rd(ra, rb, rc, rd))
        }
    };
    (impl_op_new [RegId RegId Imm12]) => {
        /// Construct the instruction from its parts.
        pub fn new(ra: RegId, rb: RegId, imm: Imm12) -> Self {
            Self(bytes_from_ra_rb_imm12(ra, rb, imm))
        }
    };
    (impl_op_new [RegId Imm18]) => {
        /// Construct the instruction from its parts.
        pub fn new(ra: RegId, imm: Imm18) -> Self {
            Self(bytes_from_ra_imm18(ra, imm))
        }
    };
    (impl_op_new [Imm24]) => {
        /// Construct the instruction from its parts.
        pub fn new(imm: Imm24) -> Self {
            Self(bytes_from_imm24(imm))
        }
    };
    (impl_op_new []) => {
        /// Construct the instruction.
        pub fn new() -> Self {
            Self([0; 3])
        }
    };

    // Generate an accessor method for each field. Recurse based on layout.
    (impl_op_accessors [RegId]) => {
        /// Access the ID for register A.
        pub fn ra(&self) -> RegId {
            ra_from_bytes(self.0)
        }
    };
    (impl_op_accessors [RegId RegId]) => {
        impl_opcodes!(impl_op_accessors [RegId]);
        /// Access the ID for register B.
        pub fn rb(&self) -> RegId {
            rb_from_bytes(self.0)
        }
    };
    (impl_op_accessors [RegId RegId RegId]) => {
        impl_opcodes!(impl_op_accessors [RegId RegId]);
        /// Access the ID for register C.
        pub fn rc(&self) -> RegId {
            rc_from_bytes(self.0)
        }
    };
    (impl_op_accessors [RegId RegId RegId RegId]) => {
        impl_opcodes!(impl_op_accessors [RegId RegId RegId]);
        /// Access the ID for register D.
        pub fn rd(&self) -> RegId {
            rd_from_bytes(self.0)
        }
    };
    (impl_op_accessors [RegId RegId Imm12]) => {
        impl_opcodes!(impl_op_accessors [RegId RegId]);
        /// Access the 12-bit immediate value.
        pub fn imm12(&self) -> Imm12 {
            imm12_from_bytes(self.0)
        }
    };
    (impl_op_accessors [RegId Imm18]) => {
        impl_opcodes!(impl_op_accessors [RegId]);
        /// Access the 18-bit immediate value.
        pub fn imm18(&self) -> Imm18 {
            imm18_from_bytes(self.0)
        }
    };
    (impl_op_accessors [Imm24]) => {
        /// Access the 24-bit immediate value.
        pub fn imm24(&self) -> Imm24 {
            imm24_from_bytes(self.0)
        }
    };
    (impl_op_accessors []) => {};

    // Generate a method for converting the instruction into its parts.
    (impl_op_unpack [RegId]) => {
        /// Convert the instruction into its parts.
        pub fn unpack(self) -> RegId {
            ra_from_bytes(self.0)
        }
    };
    (impl_op_unpack [RegId RegId]) => {
        /// Convert the instruction into its parts.
        pub fn unpack(self) -> (RegId, RegId) {
            ra_rb_from_bytes(self.0)
        }
    };
    (impl_op_unpack [RegId RegId RegId]) => {
        /// Convert the instruction into its parts.
        pub fn unpack(self) -> (RegId, RegId, RegId) {
            ra_rb_rc_from_bytes(self.0)
        }
    };
    (impl_op_unpack [RegId RegId RegId RegId]) => {
        /// Convert the instruction into its parts.
        pub fn unpack(self) -> (RegId, RegId, RegId, RegId) {
            ra_rb_rc_rd_from_bytes(self.0)
        }
    };
    (impl_op_unpack [RegId RegId Imm12]) => {
        /// Convert the instruction into its parts.
        pub fn unpack(self) -> (RegId, RegId, Imm12) {
            ra_rb_imm12_from_bytes(self.0)
        }
    };
    (impl_op_unpack [RegId Imm18]) => {
        /// Convert the instruction into its parts.
        pub fn unpack(self) -> (RegId, Imm18) {
            ra_imm18_from_bytes(self.0)
        }
    };
    (impl_op_unpack [Imm24]) => {
        /// Convert the instruction into its parts.
        pub fn unpack(self) -> Imm24 {
            imm24_from_bytes(self.0)
        }
    };
    (impl_op_unpack []) => {};

    // Generate a free function named after the $op for constructing an `Instruction`.
    (impl_op_constructor $doc:literal $Op:ident $op:ident [RegId]) => {
        #[doc = $doc]
        pub fn $op(ra: RegId) -> Instruction {
            $Op::new(ra).into()
        }
    };
    (impl_op_constructor $doc:literal $Op:ident $op:ident [RegId RegId]) => {
        #[doc = $doc]
        pub fn $op(ra: RegId, rb: RegId) -> Instruction {
            $Op::new(ra, rb).into()
        }
    };
    (impl_op_constructor $doc:literal $Op:ident $op:ident [RegId RegId RegId]) => {
        #[doc = $doc]
        pub fn $op(ra: RegId, rb: RegId, rc: RegId) -> Instruction {
            $Op::new(ra, rb, rc).into()
        }
    };
    (impl_op_constructor $doc:literal $Op:ident $op:ident [RegId RegId RegId RegId]) => {
        #[doc = $doc]
        pub fn $op(ra: RegId, rb: RegId, rc: RegId, rd: RegId) -> Instruction {
            $Op::new(ra, rb, rc, rd).into()
        }
    };
    (impl_op_constructor $doc:literal $Op:ident $op:ident [RegId RegId Imm12]) => {
        #[doc = $doc]
        pub fn $op(ra: RegId, rb: RegId, imm: Imm12) -> Instruction {
            $Op::new(ra, rb, imm).into()
        }
    };
    (impl_op_constructor $doc:literal $Op:ident $op:ident [RegId Imm18]) => {
        #[doc = $doc]
        pub fn $op(ra: RegId, imm: Imm18) -> Instruction {
            $Op::new(ra, imm).into()
        }
    };
    (impl_op_constructor $doc:literal $Op:ident $op:ident [Imm24]) => {
        #[doc = $doc]
        pub fn $op(imm: Imm24) -> Instruction {
            $Op::new(imm).into()
        }
    };
    (impl_op_constructor $doc:literal $Op:ident $op:ident []) => {
        #[doc = $doc]
        pub fn $op() -> Instruction {
            $Op::new().into()
        }
    };

    // Implement constructors and accessors for register and immediate values.
    (impl_op $doc:literal $ix:literal $Op:ident $op:ident [$($field:ident)*] $($rest:tt)*) => {
        impl $Op {
            /// The associated 8-bit Opcode value.
            pub const OPCODE: Opcode = Opcode::$Op;

            impl_opcodes!(impl_op_new [$($field)*]);
            impl_opcodes!(impl_op_accessors [$($field)*]);
            impl_opcodes!(impl_op_unpack [$($field)*]);
        }

        impl_opcodes!(impl_op_constructor $doc $Op $op [$($field)*]);

        impl From<$Op> for [u8; 3] {
            fn from($Op(arr): $Op) -> Self {
                arr
            }
        }

        impl From<$Op> for [u8; 4] {
            fn from($Op([a, b, c]): $Op) -> Self {
                [$Op::OPCODE as u8, a, b, c]
            }
        }

        impl From<$Op> for u32 {
            fn from(op: $Op) -> Self {
                u32::from_be_bytes(op.into())
            }
        }

        impl From<$Op> for Instruction {
            fn from(op: $Op) -> Self {
                Instruction::$Op(op)
            }
        }

        impl_opcodes!(impl_op $($rest)*);
    };
    (impl_op) => {};

    // Implement `TryFrom<u8>` for `Opcode`.
    (impl_opcode $($doc:literal $ix:literal $Op:ident $op:ident [$($field:ident)*])*) => {
        impl core::convert::TryFrom<u8> for Opcode {
            type Error = InvalidOpcode;
            fn try_from(u: u8) -> Result<Self, Self::Error> {
                match u {
                    $(
                        $ix => Ok(Opcode::$Op),
                    )*
                    _ => Err(InvalidOpcode),
                }
            }
        }
    };

    // Implement accessors for register and immediate values.
    (impl_instruction $($doc:literal $ix:literal $Op:ident $op:ident [$($field:ident)*])*) => {
        impl Instruction {
            /// This instruction's opcode.
            pub fn opcode(&self) -> Opcode {
                match self {
                    $(
                        Self::$Op(_) => Opcode::$Op,
                    )*
                }
            }

            // TODO:
            // - pub fn registers(&self) -> Regs
            // - pub fn immediate(&self) -> Option<u32>
        }

        impl From<Instruction> for [u8; 4] {
            fn from(inst: Instruction) -> Self {
                match inst {
                    $(
                        Instruction::$Op(op) => op.into(),
                    )*
                }
            }
        }

        impl core::convert::TryFrom<[u8; 4]> for Instruction {
            type Error = InvalidOpcode;
            fn try_from([op, a, b, c]: [u8; 4]) -> Result<Self, Self::Error> {
                match Opcode::try_from(op)? {
                    $(
                        Opcode::$Op => Ok(Self::$Op($Op([a, b, c]))),
                    )*
                }
            }
        }
    };

    // Entrypoint to the macro, generates structs, methods, opcode enum and instruction enum
    // separately.
    ($($tts:tt)*) => {
        impl_opcodes!(decl_op_struct $($tts)*);
        impl_opcodes!(decl_opcode_enum $($tts)*);
        impl_opcodes!(decl_instruction_enum $($tts)*);
        impl_opcodes!(impl_op $($tts)*);
        impl_opcodes!(impl_opcode $($tts)*);
        impl_opcodes!(impl_instruction $($tts)*);
    };
}

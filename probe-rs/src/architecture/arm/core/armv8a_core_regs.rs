use super::{RegisterDataType, RegisterDescription, RegisterFile, RegisterId, RegisterKind};

const SP: RegisterDescription = RegisterDescription {
    name: "SP",
    _kind: RegisterKind::General,
    id: RegisterId(31),
    _type: RegisterDataType::UnsignedInteger,
    size_in_bits: 64,
};

const PC: RegisterDescription = RegisterDescription {
    name: "PC",
    _kind: RegisterKind::General,
    id: RegisterId(32),
    _type: RegisterDataType::UnsignedInteger,
    size_in_bits: 64,
};

const LR: RegisterDescription = RegisterDescription {
    name: "LR",
    _kind: RegisterKind::General,
    id: RegisterId(30),
    _type: RegisterDataType::UnsignedInteger,
    size_in_bits: 64,
};

const FP: RegisterDescription = RegisterDescription {
    name: "FP",
    _kind: RegisterKind::General,
    id: RegisterId(29),
    _type: RegisterDataType::UnsignedInteger,
    size_in_bits: 64,
};

const PSTATE: RegisterDescription = RegisterDescription {
    name: "PSTATE",
    _kind: RegisterKind::General,
    id: RegisterId(33),
    _type: RegisterDataType::UnsignedInteger,
    size_in_bits: 32,
};

pub static AARCH64_REGISTER_FILE: RegisterFile = RegisterFile {
    platform_registers: &[
        RegisterDescription {
            name: "X0",
            _kind: RegisterKind::General,
            id: RegisterId(0),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X1",
            _kind: RegisterKind::General,
            id: RegisterId(1),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X2",
            _kind: RegisterKind::General,
            id: RegisterId(2),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X3",
            _kind: RegisterKind::General,
            id: RegisterId(3),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X4",
            _kind: RegisterKind::General,
            id: RegisterId(4),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X5",
            _kind: RegisterKind::General,
            id: RegisterId(5),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X6",
            _kind: RegisterKind::General,
            id: RegisterId(6),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X7",
            _kind: RegisterKind::General,
            id: RegisterId(7),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X8",
            _kind: RegisterKind::General,
            id: RegisterId(8),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X9",
            _kind: RegisterKind::General,
            id: RegisterId(9),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X10",
            _kind: RegisterKind::General,
            id: RegisterId(10),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X11",
            _kind: RegisterKind::General,
            id: RegisterId(11),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X12",
            _kind: RegisterKind::General,
            id: RegisterId(12),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X13",
            _kind: RegisterKind::General,
            id: RegisterId(13),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X14",
            _kind: RegisterKind::General,
            id: RegisterId(14),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X15",
            _kind: RegisterKind::General,
            id: RegisterId(15),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X16",
            _kind: RegisterKind::General,
            id: RegisterId(16),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X17",
            _kind: RegisterKind::General,
            id: RegisterId(17),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X18",
            _kind: RegisterKind::General,
            id: RegisterId(18),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X19",
            _kind: RegisterKind::General,
            id: RegisterId(19),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X20",
            _kind: RegisterKind::General,
            id: RegisterId(20),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X21",
            _kind: RegisterKind::General,
            id: RegisterId(21),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X22",
            _kind: RegisterKind::General,
            id: RegisterId(22),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X23",
            _kind: RegisterKind::General,
            id: RegisterId(23),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X24",
            _kind: RegisterKind::General,
            id: RegisterId(24),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X25",
            _kind: RegisterKind::General,
            id: RegisterId(25),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X26",
            _kind: RegisterKind::General,
            id: RegisterId(26),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X27",
            _kind: RegisterKind::General,
            id: RegisterId(27),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X28",
            _kind: RegisterKind::General,
            id: RegisterId(28),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X29",
            _kind: RegisterKind::General,
            id: RegisterId(29),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "X30",
            _kind: RegisterKind::General,
            id: RegisterId(30),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "SP",
            _kind: RegisterKind::General,
            id: RegisterId(31),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "PC",
            _kind: RegisterKind::General,
            id: RegisterId(32),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
    ],

    program_counter: &PC,
    stack_pointer: &SP,
    return_address: &LR,
    frame_pointer: &FP,

    argument_registers: &[
        RegisterDescription {
            name: "a0",
            _kind: RegisterKind::General,
            id: RegisterId(0),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a1",
            _kind: RegisterKind::General,
            id: RegisterId(1),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a2",
            _kind: RegisterKind::General,
            id: RegisterId(2),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a3",
            _kind: RegisterKind::General,
            id: RegisterId(3),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a4",
            _kind: RegisterKind::General,
            id: RegisterId(4),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a5",
            _kind: RegisterKind::General,
            id: RegisterId(5),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a6",
            _kind: RegisterKind::General,
            id: RegisterId(6),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a7",
            _kind: RegisterKind::General,
            id: RegisterId(7),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
    ],

    result_registers: &[
        RegisterDescription {
            name: "a0",
            _kind: RegisterKind::General,
            id: RegisterId(0),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
        RegisterDescription {
            name: "a1",
            _kind: RegisterKind::General,
            id: RegisterId(1),
            _type: RegisterDataType::UnsignedInteger,
            size_in_bits: 64,
        },
    ],

    msp: Some(&SP),
    psp: Some(&SP),
    extra: None,
    psr: Some(&PSTATE),
    fpu_registers: None, // TODO: Add fpu registers
};

use pbc_zk::{load_sbi, save_sbi, zk_compute, Sbi16, Sbi32, Sbi8, SecretVarId};

pub fn sbi() -> Sbi32 {
    load_sbi::<Sbi32>(SecretVarId::new(1))
}

pbc_zk::test_eq!(sbi(), 3, [3i32]);
pbc_zk::test_eq!(sbi(), -3213, [-3213i32]);
pbc_zk::test_eq!(sbi(), -31, [-31i32]);
pbc_zk::test_eq!(sbi(), -321242, [-321242i32]);
pbc_zk::test_eq!(sbi(), 0, [0i32]);
pbc_zk::test_eq!(sbi(), 1, [1i32]);
pbc_zk::test_eq!(sbi(), 12, [12i32]);
pbc_zk::test_eq!(sbi(), 13, [13i32]);
pbc_zk::test_eq!(sbi(), 14, [14i32]);
pbc_zk::test_eq!(sbi(), 15, [15i32]);
pbc_zk::test_eq!(sbi(), 16, [16i32]);
pbc_zk::test_eq!(sbi(), 17, [17i32]);
pbc_zk::test_eq!(sbi(), 18, [18i32]);
pbc_zk::test_eq!(sbi(), 19, [19i32]);
pbc_zk::test_eq!(sbi(), 20, [20i32]);
pbc_zk::test_eq!(sbi(), 21, [21i32], []);
pbc_zk::test_eq!(sbi(), 22, [22i32], []);

pub fn sbi_save() -> Sbi32 {
    let x = load_sbi::<Sbi32>(SecretVarId::new(1));
    save_sbi(x);
    save_sbi(x + Sbi32::from(1));
    x
}

pbc_zk::test_eq!(sbi_save(), 1, [1i32], [1i32, 2i32]);
pbc_zk::test_eq!(sbi_save(), 2, [2i32], [2i32, 3i32]);

#[zk_compute(shortname = 0x79)]
pub fn add_self() -> Sbi16 {
    load_sbi::<Sbi16>(SecretVarId::new(1)) + load_sbi::<Sbi16>(SecretVarId::new(2))
}

pbc_zk::test_eq!(add_self(), 22, [10i16, 12i16]);
pbc_zk::test_eq!(add_self(), 42, [16i16, 26i16], []);

pub fn public_add(x: u32, y: u32) -> u32 {
    x + y
}

pbc_zk::test_eq!(public_add(2, 3), 5u32);
pbc_zk::test_eq!(public_add(2, 3), 5u32, []);
pbc_zk::test_eq!(public_add(2, 3), 5u32, [], []);

pub fn pairs() -> (Sbi32, Sbi32) {
    (
        load_sbi::<Sbi32>(SecretVarId::new(1)),
        load_sbi::<Sbi32>(SecretVarId::new(2)),
    )
}

pbc_zk::test_eq!(
    {
        let p = pairs();
        p.0
    },
    1,
    [1i32, 2i32]
);
pbc_zk::test_eq!(
    {
        let p = pairs();
        p.1
    },
    2,
    [1i32, 2i32]
);

pub struct SecretType {
    v1: Sbi32,
    v2: Sbi32,
}

pub fn structs() -> SecretType {
    SecretType {
        v1: load_sbi::<Sbi32>(SecretVarId::new(1)),
        v2: load_sbi::<Sbi32>(SecretVarId::new(2)),
    }
}

pbc_zk::test_eq!(
    {
        let s = structs();
        s.v1
    },
    1,
    [1i32, 2i32]
);
pbc_zk::test_eq!(
    {
        let s = structs();
        s.v2
    },
    2,
    [1i32, 2i32]
);

pub fn unit() {
    let a = load_sbi::<Sbi32>(SecretVarId::new(1));
    save_sbi(a);
}

pbc_zk::test_eq!(unit(), (), [2i32], [2i32]);

#[derive(pbc_zk::SecretBinary)]
pub struct SecretType2 {
    v1: Sbi16,
    v2: Sbi8,
}

pub fn save_obj() {
    let a = Sbi16::from(2);
    let b = Sbi8::from(1);
    let obj = SecretType2 { v1: a, v2: b };
    save_sbi(obj);
}

pbc_zk::test_eq!(save_obj(), (), [], [0x010002i32]);

pub fn identity(_x: Sbi8) {
    save_sbi::<Sbi8>(Sbi8::from(0));
    save_sbi::<Sbi8>(Sbi8::from(0));
    save_sbi::<Sbi8>(Sbi8::from(0));
    save_sbi::<Sbi8>(Sbi8::from(0));
}

pbc_zk::test_eq!(identity(Sbi8::from(1)), (), [0; 0], [0i8; 4]);

pub fn equal(v1: [i32; 11], v2: [i32; 11]) -> bool {
    v1 == v2
}

pbc_zk::test_eq!(equal([0; 11], [0; 11]), true);
pbc_zk::test_eq!(
    equal(
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    ),
    true
);

struct Tuple {
    v1: Sbi8,
    v2: Sbi8,
    v3: Sbi8,
}

fn sum_everything() -> Tuple {
    // Initialize state
    let mut counts = Tuple {
        v1: Sbi8::from(0),
        v2: Sbi8::from(0),
        v3: Sbi8::from(0),
    };
    counts.v1 = Sbi8::from(0);
    counts.v2 = Sbi8::from(1);
    counts.v3 = Sbi8::from(2);
    counts
}

pbc_zk::test_eq!(
    {
        let t = sum_everything();
        t.v1 == Sbi8::from(0) && t.v2 == Sbi8::from(1) && t.v3 == Sbi8::from(2)
    },
    true
);

pub fn sum_everything2() -> [Sbi8; 8] {
    // Initialize state
    let mut counts: [Sbi8; 8] = [Sbi8::from(0); 8];

    // Count each variable
    for variable_id in pbc_zk::secret_variable_ids() {
        let var = load_sbi::<Sbi8>(variable_id);
        println!("{:?}", var);
        for value in 0i8..8i8 {
            let idx = value as usize;
            if var == Sbi8::from(value) {
                counts[idx] = counts[idx] + Sbi8::from(1);
            }
        }
    }

    counts
}

pbc_zk::test_eq!(
    sum_everything2()
        == [
            Sbi8::from(0x0b),
            Sbi8::from(0x03),
            Sbi8::from(0x05),
            Sbi8::from(0x06),
            Sbi8::from(0x00),
            Sbi8::from(0x03),
            Sbi8::from(0x03),
            Sbi8::from(0x01)
        ],
    true,
    [
        0i8, 2i8, 1i8, 3i8, 0i8, 2i8, 3i8, 0i8, 1i8, 5i8, 0i8, 6i8, 0i8, 1i8, 2i8, 0i8, 3i8, 0i8,
        5i8, 6i8, 0i8, 3i8, 2i8, 6i8, 0i8, 3i8, 0i8, 7i8, 0i8, 3i8, 5i8, 2i8
    ]
);

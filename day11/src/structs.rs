use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub struct Monke {
    pub items: Vec<i64>,
    pub divident: i64,
    pub dest_t: i64,
    pub dest_f: i64,
    pub op: String,
}

#[derive(Debug, Clone)]
pub struct BigMonke {
    pub items: Vec<BigUint>,
    pub divident: BigUint,
    pub dest_t: i64,
    pub dest_f: i64,
    pub op: String,
}

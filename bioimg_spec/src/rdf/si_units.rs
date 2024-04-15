use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiUnit {
    pub multiplier: Option<SiMultiplier>,
    pub measure: SiMesaure,
    pub exponent: i32,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SiMultiplier {
    Q,
    R,
    Y,
    Z,
    E,
    P,
    T,
    G,
    M,
    k,
    h,
    da,
    d,
    c,
    m,
    #[serde(rename = "µ")]
    micro,
    n,
    p,
    f,
    a,
    z,
    y,
    r,
    q,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SiMesaure {
    m,
    g,
    s,
    A,
    K,
    mol,
    cd,
    Hz,
    N,
    Pa,
    J,
    W,
    C,
    V,
    F,
    #[serde(rename = "Ω")]
    Ohm,
    S,
    Wb,
    T,
    H,
    lm,
    lx,
    Bq,
    Gy,
    Sv,
    kat,
    l,
    L,
}

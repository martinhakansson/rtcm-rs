use rand::Rng;

pub struct ValGen<FR, LR, RR>
where
    FR: Rng,
    LR: Rng,
    RR: Rng,
{
    pub field_rng: FR,
    pub len_rng: LR,
    pub rng_rng: RR,
}

impl<FR, LR, RR> ValGen<FR, LR, RR>
where
    FR: Rng,
    LR: Rng,
    RR: Rng,
{
    pub fn new(field_rng: FR, len_rng: LR, rng_rng: RR) -> Self {
        ValGen {
            field_rng,
            len_rng,
            rng_rng,
        }
    }
}

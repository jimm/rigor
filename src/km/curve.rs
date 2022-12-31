pub struct Curve {
    pub name: String,
    pub short_name: String,
    pub curve: [u8; 128],
}

impl Curve {
    pub fn new(name: &str, short_name: &str, bytes: [u8; 128]) -> Curve {
        Curve {
            name: name.to_string(),
            short_name: short_name.to_string(),
            curve: bytes,
        }
    }

    pub fn short_name(&self) -> &str {
        &self.short_name[..]
    }

    fn linear() -> Curve {
        let mut bytes: [u8; 128] = [0; 128];

        for i in 0..128 {
            bytes[i] = i as u8
        }
        return Self::new("Linear", "lin", bytes);
    }

    fn exp() -> Curve {
        let mut bytes: [u8; 128] = [0; 128];

        for i in 0..128 {
            let mut val = i * i / 127;
            if i > 0 && val == 0 {
                val = 1
            }
            bytes[i] = val as u8
        }
        return Self::new("Exponential", "exp", bytes);
    }

    fn half_exp() -> Curve {
        let mut bytes: [u8; 128] = [0; 128];

        for i in 0..128 {
            let mut exponential = i * i / 127;
            if i > 0 && exponential == 0 {
                exponential = 1
            }
            bytes[i] = ((i + exponential) / 2) as u8
        }
        return Self::new("Half Exponential", "exp/2", bytes);
    }

    fn inv_exp() -> Curve {
        let mut bytes: [u8; 128] = [0; 128];

        for i in 0..128 {
            let inv = 127 - i;
            bytes[i] = (127 - ((inv * inv) / 127)) as u8
        }
        return Self::new("Inverse Exponential", "-exp", bytes);
    }

    fn half_inv_exp() -> Curve {
        let mut bytes: [u8; 128] = [0; 128];

        for i in 0..128 {
            let inv = 127 - i;
            let inv_exp = 127 - ((inv * inv) / 127);
            bytes[i] = ((i + inv_exp) / 2) as u8
        }
        return Self::new("Half Inverse Exponential", "-exp/2", bytes);
    }

    pub fn build_curves() -> Vec<Curve> {
        let mut v = Vec::new();
        v.push(Self::linear());
        v.push(Self::exp());
        v.push(Self::half_exp());
        v.push(Self::inv_exp());
        v.push(Self::half_inv_exp());
        return v;
    }
}

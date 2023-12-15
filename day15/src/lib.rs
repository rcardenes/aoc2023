use std::fmt::Debug;

#[derive(Clone)]
pub struct Lens {
    label: String,
    f: u8,
}

impl Debug for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.f)
    }
}

#[derive(Clone)]
pub struct LensBox {
    lenses: Vec<Lens>
}

impl Debug for LensBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: Vec<String> = self.lenses.iter().map(|l| format!("{:?}", l)).collect();
        write!(f, "{}", res.join(" "))
    }
}

impl LensBox {
    pub fn new() -> Self {
        LensBox { lenses: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.lenses.is_empty()
    }

    pub fn remove(&self, label: &str) -> LensBox {
        LensBox {
            lenses: self.lenses.clone().into_iter().filter(|l| l.label != label).collect()
        }
    }

    pub fn replace(&self, label: &str, f: u8) -> LensBox {
        let new_lens = Lens { label: label.to_string(), f };
        if self.lenses.iter().any(|l| l.label == label) {
            LensBox {
                lenses: self.lenses
                    .clone()
                    .into_iter()
                    .map(|l| if l.label == label { new_lens.clone() } else { l })
                    .collect()
            }
        } else {
            let mut res = LensBox { lenses: self.lenses.clone() };
            res.lenses.push(new_lens);
            res
        }
    }

    pub fn power(&self) -> u32 {
        self.lenses.iter()
            .enumerate()
            .fold(0, |acc, (k, l)| {
                (l.f as u32) * (k as u32 + 1) + acc
            })
    }
}

pub fn hash_algo(s: &str) -> u8 {
    s.bytes()
        .fold(0u16, |acc, ascii| {
            ((acc + (ascii as u16)) * 17) % 256
        }) as u8
}

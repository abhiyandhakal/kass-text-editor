use crate::kass::Kass;

pub struct Group<F>
where
    F: Fn(&mut Kass),
{
    sequence: String,
    func: Option<F>,
}

impl<F> Group<F>
where
    F: Fn(&mut Kass),
{
    pub fn new(sequence: &str, func: F) -> Self {
        Group {
            sequence: sequence.to_string(),
            func: Some(func),
        }
    }

    pub fn call_stored_function(&self, kass: &mut Kass) {
        if let Some(ref func) = self.func {
            func(kass);
        }
    }
}

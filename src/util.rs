pub trait SetMin{
    fn set_min(&mut self, other: Self);
}

impl<T: Ord> SetMin for T {
    fn set_min(&mut self, other: Self) {
        if *self > other {
            *self = other;
        }
    }
}

pub trait SetMax {
    fn set_max(&mut self, other: &Self);
}

impl<T: Ord + Clone> SetMax for T {
    fn set_max(&mut self, other: &Self) {
        if *self > *other {
            *self = other.clone();
        }
    }
}

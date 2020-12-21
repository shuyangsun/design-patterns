use crate::traits::composition::Composition;

pub trait Compositor: Clone {
    fn set_composition(&mut self, composition: &dyn Composition);
    fn compose(&self);
}

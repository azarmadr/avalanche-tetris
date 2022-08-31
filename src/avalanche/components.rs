use {
    bevy::prelude::*,
    duplicate::*,
    // enum_dispatch::enum_dispatch,
    std::fmt::{self, Display, Formatter},
};

// Single Tuples
duplicate! {[component t; [Idx] [usize];]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component,Deref,DerefMut, Copy, Clone, Debug)]
pub struct component(pub t);
impl Display for component {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(v) = self;
        write!(f, "component: {v}")
    }
}}

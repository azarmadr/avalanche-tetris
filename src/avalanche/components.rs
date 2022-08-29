use {
    super::Sq,
    bevy::prelude::*,
    duplicate::*,
    // enum_dispatch::enum_dispatch,
    std::{
        fmt::{self, Display, Formatter},
        ops::Deref,
    },
};

duplicate! {[component t; [Idx] [Sq];]
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Copy, Clone, Debug)]
pub struct component(pub usize,pub t);
impl Deref for component {
    type Target = t;
    fn deref(&self) -> &t {
        &self.1
    }
}
impl Display for component {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(v, u) = self;
        write!(f, "component: {v} {u}")
    }
}
}
impl Idx {
    pub const fn from2d(x: usize, y: usize, width: usize) -> Self {
        Self(x + width * y, false)
    }
}

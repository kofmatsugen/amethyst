//! ECS rendering bundle

use amethyst_assets::Processor;
use amethyst_core::bundle::{ECSBundle, Result};
use amethyst_core::specs::prelude::{DispatcherBuilder, World};
use std::hash::Hash;
use std::marker::PhantomData;

use super::*;

/// UI bundle
///
/// Will register all necessary components and systems needed for UI, along with any resources.
/// The generic types A and B represent the A and B generic parameter of the InputHandler<A,B>.
///
/// Will fail with error 'No resource with the given id' if the InputBundle is not added.
pub struct UiBundle<A, B> {
    _marker1: PhantomData<A>,
    _marker2: PhantomData<B>,
}

impl<A, B> UiBundle<A, B> {
    /// Create a new UI bundle
    pub fn new() -> Self {
        UiBundle {
            _marker1: PhantomData,
            _marker2: PhantomData,
        }
    }
}

impl<'a, 'b, A, B> ECSBundle<'a, 'b> for UiBundle<A, B>
where
    A: Send + Sync + Eq + Hash + Clone + 'static,
    B: Send + Sync + Eq + Hash + Clone + 'static,
{
    fn build(
        self,
        _: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        Ok(builder
            .with(Processor::<FontAsset>::new(), "font_processor", &[])
            .with(UiSystem::new(), "ui_system", &["font_processor"])
            .with(ResizeSystem::new(), "ui_resize_system", &[])
            .with(UiMouseSystem::<A, B>::new(), "ui_mouse_system", &[])
            .with(UiLayoutSystem::new(), "ui_layout", &["ui_system"])
            .with(
                UiParentSystem::default(),
                "ui_parent",
                &["transform_system", "ui_layout"],
            ))
    }
}

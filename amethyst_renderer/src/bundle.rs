//! ECS rendering bundle

use amethyst_core::bundle::{ECSBundle, Result, ResultExt};
use amethyst_core::specs::prelude::{DispatcherBuilder, World};
use config::DisplayConfig;
use pipe::{PipelineBuild, PolyPipeline};
use system::RenderSystem;
use visibility::VisibilitySortingSystem;

/// Rendering bundle
///
/// Will register all necessary components needed for rendering, along with any resources.
/// Will also register asset contexts with the asset `Loader`, and add systems for merging
/// `AssetFuture` into its related component.
///
/// Will register `TransparentSortingSystem`, with name `transparent_sorting_system` if sorting is
/// requested.
///
pub struct RenderBundle<'a, B, P>
where
    B: PipelineBuild<Pipeline = P>,
    P: PolyPipeline,
{
    pipe: B,
    config: Option<DisplayConfig>,
    visibility_sorting: Option<&'a [&'a str]>,
}

impl<'a, B, P> RenderBundle<'a, B, P>
where
    B: PipelineBuild<Pipeline = P>,
    P: PolyPipeline,
{
    /// Create a new render bundle
    pub fn new(pipe: B, config: Option<DisplayConfig>) -> Self {
        RenderBundle {
            pipe,
            config,
            visibility_sorting: None,
        }
    }

    /// Enable transparent mesh sorting, with the given dependencies
    pub fn with_visibility_sorting(mut self, dep: &'a [&'a str]) -> Self {
        self.visibility_sorting = Some(dep);
        self
    }
}

impl<'a, 'b, 'c, B: PipelineBuild<Pipeline = P>, P: 'b + PolyPipeline> ECSBundle<'a, 'b>
    for RenderBundle<'c, B, P>
{
    fn build(
        self,
        world: &mut World,
        mut builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        use amethyst_core::specs::prelude::RunNow;
        let mut system =
            RenderSystem::build(self.pipe, self.config).chain_err(|| "Renderer error!")?;
        system.setup(&mut world.res);
        if let Some(dep) = self.visibility_sorting {
            builder = builder.with(
                VisibilitySortingSystem::new(),
                "visibility_sorting_system",
                dep,
            );
        };
        Ok(builder.with_thread_local(system))
    }
}

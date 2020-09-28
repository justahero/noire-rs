use crate::RenderResourceContext;

pub trait RenderContext {
    fn context(&self) -> &dyn RenderResourceContext;
    fn context_mut(&mut self) -> &mut dyn RenderResourceContext;
}

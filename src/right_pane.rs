use orbtk::prelude::*;
use crate::*;

#[derive(Default, AsAny)]
pub struct RightPaneState {
}

impl RightPaneState {
}

impl State for RightPaneState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(RightPane<RightPaneState> {
    open: bool
});

impl Template for RightPane {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("RightPane")
        .child(
            
        )
    }
}
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
            Stack::create()
                .orientation("vertical")
                .child(
                    Container::create()
                        .height(200.0)
                        .width(200.0)
                        .child(
                            TextBlock::create()
                                .text("ProfilePicture")
                                .build(ctx)
                        )
                        .build(ctx)
                )
                .child(
                    TextBlock::create()
                        .text("Profile Info")
                        .build(ctx)
                )
                .build(ctx)
        )
    }
}
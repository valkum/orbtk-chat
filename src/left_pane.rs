use orbtk::prelude::*;
use crate::*;
use std::cell::Cell;


#[derive(Default, AsAny)]
pub struct LeftPaneState {
}

impl State for LeftPaneState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context<'_>) {
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(LeftPane<LeftPaneState> {
    open: bool
});

impl Template for LeftPane {
    /// (Profilepic) Name
    /// > Direct
    /// > Room
    ///   # Room 1
    ///   # Room 2
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("LeftPane")
            // .width(100.0)
            .child(
                Container::create()
                    .padding((0.0, 28.0, 0.0, 0.0))
                    .background("#25272A")
                    .id("leftPanel-container")
                    .child(
                        Stack::create()
                            .orientation("vertical")
                            .child(
                                TextBlock::create()
                                    .font_size(20.0)
                                    .text("My Username")
                                    .build(ctx)
                            )
                            .child(
                                TextBlock::create()
                                    .text("Channels")
                                    .build(ctx)
                            )
                            .child(
                                RoomList::create()
                                    .build(ctx)
                            )
                            .child(
                                TextBlock::create()
                                    .text("Direct Messages")
                                    .build(ctx)
                            )
                            .child(
                                TextBlock::create()
                                    .text("User Username")
                                    .build(ctx)
                            )
                            .build(ctx)
                    )
                    .build(ctx)
            )
    }
}
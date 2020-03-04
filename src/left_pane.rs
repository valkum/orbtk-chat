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
            .width(100.0)
            .child(
                Container::create()
                    .padding((0.0, 10.0, 0.0, 0.0))
                    .attach(Grid::column(0))
                    .attach(Grid::row(0))
                    .id("leftPanel-container")
                    .child(
                        RoomList::create()
                            .build(ctx)
                    )
                    .build(ctx)
            )
    }
}
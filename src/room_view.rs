use orbtk::prelude::*;
use crate::*;

#[derive(Default, AsAny)]
pub struct RoomViewState {
}

impl RoomViewState {
}

impl State for RoomViewState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(RoomView<RoomViewState> {
    open: bool
});

impl Template for RoomView {
    /// RoomHeader
    /// MessagePanel(Show RightPane when selected in RoomHeader)
    /// MessageComposer
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("RoomView")
        .child(
            Grid::create()
                .horizontal_alignment("start")
                .rows(
                    Rows::create()
                        .row("auto")
                        .row("*")
                        .row("auto")
                        .build()
                )
                .child(
                    RoomHeader::create()
                        .attach(Grid::row(0))
                        .build(ctx)
                )
                .child(
                    MessagePanel::create()
                        .attach(Grid::row(1))
                        .build(ctx)
                )
                .child(
                    MessageComposer::create()
                        .attach(Grid::row(2))
                        .build(ctx)
                )
                .build(ctx)
        )
    }
}
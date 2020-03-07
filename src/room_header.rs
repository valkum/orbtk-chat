use orbtk::prelude::*;
use crate::*;

#[derive(Default, AsAny)]
pub struct RoomHeaderState {
}

impl RoomHeaderState {
}

impl State for RoomHeaderState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(RoomHeader<RoomHeaderState> {
    open: bool
});

impl Template for RoomHeader {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("RoomHeader")
        .child(
            Container::create()
            .height(50.0)
            .child(
                Stack::create()
                    .orientation("horizontal")
                    .child(
                        TextBlock::create()
                            .horizontal_alignment("start")
                            .text("Channel 1")
                            .font_size(20.0)
                            .build(ctx)
                    )
                    .child(
                        TextBlock::create()
                            .horizontal_alignment("start")
                            .text("Searchbox")
                            .build(ctx)
                    )
                    .build(ctx)
            )
            .build(ctx)
        )
    }
}
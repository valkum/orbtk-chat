use orbtk::prelude::*;
use crate::*;

#[derive(Default, AsAny)]
pub struct RoomListState {
}

impl RoomListState {
}

impl State for RoomListState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}

widget!(RoomList<RoomListState> {
});


impl Template for RoomList {
    /// # Room 1
    /// # Room 2
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("RoomList")
        .child(
            Container::create()
                .padding((0.0, 10.0, 0.0, 0.0))
                .attach(Grid::column(0))
                .attach(Grid::row(0))
                .id("roomList-container")
                .child(
                    ListView::create()
                        .items_builder(move |bc, index| {
                            Container::create()
                                .padding(20.)
                                .child(
                                    TextBlock::create().text(format!("Channel {}", index)).build(bc)
                                )
                                .build(bc)
                            
                        })
                        .count(5)
                        .build(ctx)
                )
                .build(ctx)
        )
    }
}
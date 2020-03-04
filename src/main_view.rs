use orbtk::prelude::*;
use crate::*;

#[derive(Default, AsAny)]
pub struct MainViewState {
}

impl MainViewState {
}

impl State for MainViewState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(MainView<MainViewState> {
});

impl Template for MainView {
    /// LeftPane | RoomView
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
        .child(
            Grid::create()
                .columns(
                    Columns::create()
                        .column("auto")
                        .column("*")
                        .build(),
                )
                .id("app")
                .child(
                    LeftPane::create()
                        .attach(Grid::column(0))
                        .build(ctx)
                )
                .child(
                    RoomView::create()
                        .attach(Grid::column(1))
                        .build(ctx)
                )
                .build(ctx)
        )
    }
}
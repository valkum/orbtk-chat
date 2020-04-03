use orbtk::prelude::*;
use crate::*;


#[derive(Default, AsAny)]
pub struct LeftMostPaneState {
}

impl State for LeftMostPaneState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context<'_>) {
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(LeftMostPane<LeftMostPaneState> {
});

impl Template for LeftMostPane {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("LeftMostPane")
            .child(
                Container::create()
                    .background("#000000")
                    .child(
                        Grid::create()
                            .rows(
                                Rows::create()
                                    .row("*")
                                    .row("auto")
                                    .build(),
                            )
                            .child(
                                Stack::create()
                                    .attach(Grid::row(0))
                                    .id("leftMostPanel-container")
                                    .horizontal_alignment("center")
                                    .child(
                                        Container::create()
                                        .height(20.0)
                                        .child(
                                            WindowButtons::create().build(ctx)
                                        )
                                        .build(ctx)
                                    )
                                    .child(
                                        IconButton::create()
                                            .icon("Icon")
                                            .class("channel-button")
                                            .build(ctx)
                                    )
                                    .child(
                                        TextBlock::create()
                                            .text("Icon")
                                            .build(ctx)
                                    )
                                    .build(ctx)
                            )
                            .child(
                                Stack::create()
                                    .vertical_alignment("center")
                                    .child(
                                        TextBlock::create()
                                            .attach(Grid::row(1))
                                            .text("+")
                                            .build(ctx)
                                    )
                                    .build(ctx)
                            )
                            .build(ctx)
                    )
                    .build(ctx)
            )
    }
}
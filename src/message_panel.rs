use orbtk::prelude::*;
use crate::*;

#[derive(Default, AsAny)]
pub struct MessagePanelState {
}

impl MessagePanelState {
}

impl State for MessagePanelState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(MessagePanel<MessagePanelState> {
    open: bool
});

impl Template for MessagePanel {
    // bodyView
    // when panelView
    // bodyView | panelView
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MessagePanel")
        .child(
            TextBlock::create()
                .text("MessagePanel")
                .build(ctx)
        )
    }
}
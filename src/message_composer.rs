use orbtk::prelude::*;
use crate::*;

#[derive(Default, AsAny)]
pub struct MessageComposerState {
}

impl MessageComposerState {
}

impl State for MessageComposerState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context<'_>) {
    }
}
widget!(MessageComposer<MessageComposerState> {
    open: bool
});

impl Template for MessageComposer {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MessageComposer")
        .child(
            Container::create()
            .height(50.0)
            .child(
                TextBox::create()
                .build(ctx)
            )
            .build(ctx)
        )
    }
}
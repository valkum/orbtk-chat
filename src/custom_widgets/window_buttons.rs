use orbtk::prelude::*;
use orbtk::widgets::behaviors::MouseBehavior;
use crate::*;
use std::cell::Cell;



widget!(
    WindowButton: MouseHandler {
        background: Brush,
        radius: f64,
        pressed: bool
    }
);

impl Template for WindowButton {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        
        self.name("WindowButton")
            .element("window-button")
            .radius(99.0)
            .pressed(false)
            .child(
                // Add padding to get an inside border.
                Container::create()
                // Todo compute padding on the fly.
                    .padding(11.0)
                    .child(
                        MouseBehavior::create()
                            .pressed(id)
                            .enabled(id)
                            .target(id.0)
                            .child(
                                Container::create()
                                    .background(id)
                                    .border_radius(("radius", id))
                                    .border_brush("transparent")
                                    .height(2.0)
                                    .width(2.0)
                                    .build(ctx),
                            )
                            .build(ctx)
                    )
                    .build(ctx),
            )
    }
}


#[derive(Debug, Copy, Clone)]
enum Action {
    Close,
    Maximize,
    Minimize,
}
#[derive(Default, AsAny)]
pub struct WindowButtonsState {
    action: Cell<Option<Action>>,
}

impl WindowButtonsState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }
}

impl State for WindowButtonsState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("enabled") {
            return;
        }

        if let Some(action) = self.action.get() {
            match action {
                Action::Close => {
                    println!("Action::Close pressed");
                    let parent = ctx.entity_of_parent().unwrap();
                    ctx.push_event_by_entity(SystemEvent::Quit, parent)
                }
                Action::Maximize => {
                    
                }
                Action::Minimize => {
                    
                }
            };
            self.action.set(None);
        }
    }
}

widget!(
    WindowButtons<WindowButtonsState> {}
);

impl Template for WindowButtons {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("WindowButtons")
        .child(
            Stack::create()
            .orientation("horizontal")
            .element("window-buttons")
            .child(
                WindowButton::create()
                    .radius(id)
                    .background("#ff0000")
                    .id("window-close")
                    .on_click(move |states, _| -> bool {
                        states
                            .get::<WindowButtonsState>(id)
                            .action(Action::Close);
                        true
                    })
                    .build(ctx)
            )
            .child(
                WindowButton::create()
                    .radius(id)
                    .background("#00ff00")
                    .id("window-minimize")
                    .on_click(move |states, _| -> bool {
                        states
                            .get::<WindowButtonsState>(id)
                            .action(Action::Maximize);
                        true
                    })
                    .build(ctx)
            )
            .child(
                WindowButton::create()
                    .radius(id)
                    .background("#0000ff")
                    .id("window-maximize")
                    .on_click(move |states, _| -> bool {
                        states
                            .get::<WindowButtonsState>(id)
                            .action(Action::Minimize);
                        true
                    })
                    .build(ctx)
            )
            .build(ctx)
        )
    }
}

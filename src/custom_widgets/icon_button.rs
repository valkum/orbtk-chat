use orbtk::widgets::behaviors::MouseBehavior;
use orbtk::prelude::*;

widget!(
    /// The `Button` widget can be clicked by user. It's used to perform an action.
    ///
    /// **CSS element:** `button`
    IconButton: MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the icon property.
        icon: String,

        /// Sets or shares the icon brush property.
        icon_brush: Brush,

        /// Sets or share the icon font size property.
        icon_size: f64,

        /// Sets or shares the icon font property.
        icon_font: String,

        /// Sets or shares the pressed property.
        pressed: bool
    }
);

impl Template for IconButton {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("IconButton")
            .element("icon-button")
            .height(18.0)
            .width(18.0)
            .background(colors::LYNCH_COLOR)
            .border_radius(36.0)
            .border_width(0.0)
            .border_brush("transparent")
            .icon("")
            .icon_font("Material Icons")
            .icon_size(7.0)
            .icon_brush(colors::LINK_WATER_COLOR)
            .pressed(false)
            .child(
                Container::create()
                    .padding(10.0)
                    .child(
                        MouseBehavior::create()
                            .pressed(id)
                            .enabled(id)
                            .target(id.0)
                            .child(
                                Container::create()
                                    .background(id)
                                    .border_radius(id)
                                    .border_width(id)
                                    .border_brush(id)
                                    .opacity(id)
                                    .child(
                                        FontIconBlock::create()
                                            .vertical_alignment("center")
                                            .icon(id)
                                            .icon_brush(id)
                                            .icon_size(id)
                                            .icon_font(id)
                                            .opacity(id)
                                            .build(ctx),
                                    )
                                    .build(ctx)
                            )
                            .build(ctx)
                    )
                    .build(ctx),
            )
    }
}

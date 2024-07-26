use vizia::{
    icons::{ICON_CURSOR_TEXT, ICON_ERASER, ICON_PENCIL, ICON_PENCIL_SHARE, ICON_POINTER, ICON_SETTINGS, ICON_SLICE},
    prelude::*,
};

#[derive(Lens)]
pub struct TopBar {}

impl TopBar {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}
            .build(cx, |cx| {

                IconButton::new(cx, ICON_POINTER);

                IconButton::new(cx, ICON_CURSOR_TEXT);
                // Edit
                IconButton::new(cx, ICON_PENCIL);

                // Edit
                IconButton::new(cx, ICON_ERASER);

                // Edit
                IconButton::new(cx, ICON_SLICE);

                // Edit
                

                
                // ToggleButton::new(cx, "\u{eb20}");
                Submenu::new(
                    cx,
                    |cx| Icon::new(cx, ICON_SETTINGS),
                    |cx| {
                        MenuButton::new(
                            cx,
                            |_| println!("New"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, "New");
                                    Label::new(cx, "Ctrl + N").class("shortcut");
                                })
                            },
                        );
                        MenuButton::new(
                            cx,
                            |_| println!("Open"),
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, "Open");
                                    Label::new(cx, "Ctrl + O").class("shortcut");
                                })
                            },
                        );
                        Submenu::new(
                            cx,
                            |cx| Label::new(cx, "Open Recent"),
                            |cx| {
                                MenuButton::new(
                                    cx,
                                    |_| println!("Doc 1"),
                                    |cx| Label::new(cx, "Doc 1"),
                                );
                                Submenu::new(
                                    cx,
                                    |cx| Label::new(cx, "Doc 2"),
                                    |cx| {
                                        MenuButton::new(
                                            cx,
                                            |_| println!("Version 1"),
                                            |cx| Label::new(cx, "Version 1"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| println!("Version 2"),
                                            |cx| Label::new(cx, "Version 2"),
                                        );
                                        MenuButton::new(
                                            cx,
                                            |_| println!("Version 3"),
                                            |cx| Label::new(cx, "Version 3"),
                                        );
                                    },
                                );
                                MenuButton::new(
                                    cx,
                                    |_| println!("Doc 3"),
                                    |cx| Label::new(cx, "Doc 3"),
                                );
                            },
                        );
                        Divider::new(cx);
                        MenuButton::new(cx, |_| println!("Save"), |cx| Label::new(cx, "Save"));
                        MenuButton::new(
                            cx,
                            |_| println!("Save As"),
                            |cx| Label::new(cx, "Save As"),
                        );
                        Divider::new(cx);
                        MenuButton::new(cx, |_| println!("Quit"), |cx| Label::new(cx, "Quit"));
                    },
                )
                .class("menu");
            })
            .layout_type(LayoutType::Row)
    }
}

impl View for TopBar {
    fn element(&self) -> Option<&'static str> {
        Some("topbar")
    }
}

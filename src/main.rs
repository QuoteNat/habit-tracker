use adw::prelude::*;

use adw::{ActionRow, ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Box, ListBox, Orientation};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .selectable(false)
            .title("Good habit 1")
            .build();
        row.connect_activated(|_| {
            eprintln!("Good job!");
        });

        let bad_row = ActionRow::builder()
            .activatable(true)
            .selectable(false)
            .title("Bad habit 1")
            .build();
        bad_row.connect_activated(|_| {
            eprintln!("Bad job!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            // the content class makes the list look nicer
            .css_classes(vec![String::from("content")])
            .build();
        list.append(&row);

        let bad_list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            // the content class makes the list look nicer
            .css_classes(vec![String::from("content")])
            .build();
        bad_list.append(&bad_row);      

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        let columns = Box::new(Orientation::Horizontal, 0);
        
        columns.append(&list);
        columns.append(&bad_list);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("First App", ""))
                .build(),
        );
        content.append(&columns);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}
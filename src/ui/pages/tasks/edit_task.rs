use iced::{
    border::Radius, widget::{
        button, column, container, svg, text, text_editor, text_input, Row, Space
    }, Background, Border, Color, Element, Length, Padding
};

use crate::{icons::arrow_back_icon, ui::{components::headers::button_n_text, styles::ToDoTheme, Message, Page, ToDo}};

pub fn func(todo: &ToDo) -> Element<Message> {
    let mut buttons = Vec::new();
    for i in 0..5{
        buttons.push(
            container(
                button("")
                .on_press(Message::PriorityEditChange(i))
                .style(move |theme, status| ToDo::priority_button(
                    ToDo::get_theme(theme), todo.edit.priority, 
                    status, i
                )).width(30).height(30)
            ).center_x(Length::Fill).into(),
        );
    };

    container(
        column![
            button_n_text(
                svg(svg::Handle::from_memory(arrow_back_icon()))
                    .width(36).height(44)
                    .style(|theme, _| ToDo::svg_icon(
                        ToDo::get_theme(theme), Color::parse("#efefef").unwrap()
                    )),
                Message::ChangePage(Page::TasksList), String::from("Изменить задачу")),
            container(
                column![
                    container(
                        column![
                        text_input("Имя", &todo.edit.name)
                            .on_input(Message::NameEditType)
                            .width(300)
                            .padding(Padding::new(10.0)),
                        text_editor(&todo.edit.description)
                            .on_action(Message::DescriptionEditType)
                            .placeholder("Описание")
                            .width(300)
                            .padding(Padding::new(10.0))
                            .height(128)
                        ].spacing(20)
                    ).center_x(Length::Fill),
                    container(
                    column![
                        container(
                            text("Приоритет").size(12)
                        ).center_x(Length::Fill)
                        .padding(Padding::bottom(Padding::default(), 10.0)),
                        container(
                            Row::from_vec(buttons).width(320)
                        ).center_x(Length::Fill)
                    ]).padding(Padding::new(20.0)),
                    container(
                        button("Изменить").padding(Padding::new(10.0))
                        .on_press(Message::EditTask)
                        .style(|theme, status| button::Style { 
                            text_color: match ToDo::get_theme(theme) {
                                ToDoTheme::Dark => Color::from_rgb(0.1, 0.1, 0.1),
                                ToDoTheme::Light => Color::WHITE
                            },
                            background: Some(Background::Color(
                                match status{
                                    button::Status::Hovered => ToDo::primary_color(),
                                    _ => ToDo::primary_color().scale_alpha(0.95)
                                })),
                            border: Border{
                                radius: Radius::new(8),
                                ..Default::default()
                            },
                            ..Default::default() 
                        })
                    ).center_x(Length::Fill),
                    Space::new(10, 10),
                    container(
                        text(&todo.edit.error)
                        .color(Color::from_rgb(1.0, 0.0, 0.0))
                    ).center_x(Length::Fill),
                    container(
                        text(&todo.edit.success)
                        .color(Color::from_rgb(0.0, 1.0, 0.0))
                    ).center_x(Length::Fill)
                ]
            ).center(Length::Fill)
            .padding(Padding::bottom(Padding::default(), 50.0)),
        ]
    ).into()
}
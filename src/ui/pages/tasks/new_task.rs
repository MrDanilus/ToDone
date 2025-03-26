use iced::{
    border::Radius, widget::{
        button, column, container, text, Row, Space
    }, Background, Border, Color, Element, Length, Padding
};

use crate::{
    core::update::{
        tasks::TasksMsg, Message
    }, icons::arrow_back_icon, ui::{
        components::{headers::button_n_text, inputs}, 
        styles::ToDoTheme, Page, ToDo
    }
};

pub fn func(todo: &ToDo) -> Element<Message> {
    let mut buttons = Vec::new();
    for i in 0..5{
        buttons.push(
            container(
                button("")
                .on_press(
                    Message::Tasks(TasksMsg::PriorityCreateChange(i))
                ).style(move |theme, status| ToDo::priority_button(
                    ToDo::get_theme(theme), todo.create.priority, 
                    status, i
                )).width(30).height(30)
            ).center_x(Length::Fill).into(),
        );
    };

    container(
        column![
            button_n_text(
                arrow_back_icon(), Message::ChangePage(Page::TasksList),
                String::from("Новая задача")
            ),
            container(
                column![
                    container(
                        column![
                        inputs::input(
                            "Имя", todo.create.name.clone(), 
                            |str| Message::Tasks(TasksMsg::NameCreateType(str))
                        ).width(300).padding(Padding::new(10.0)),
                        inputs::editor(
                            "Описание", &todo.create.description,
                            |action| Message::Tasks(TasksMsg::DescriptionCreateType(action))
                        ).width(300).height(128)
                        .padding(Padding::new(10.0))
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
                        button("Создать").padding(Padding::new(10.0))
                        .on_press(
                            Message::Tasks(TasksMsg::CreateTask)
                        ).style(|theme, status| button::Style { 
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
                        text(&todo.create.error)
                        .color(Color::from_rgb(1.0, 0.0, 0.0))
                    ).center_x(Length::Fill),
                    container(
                        text(&todo.create.success)
                        .color(Color::from_rgb(0.0, 1.0, 0.0))
                    ).center_x(Length::Fill)
                ]
            ).center(Length::Fill)
            .padding(Padding::bottom(Padding::default(), 50.0)),
        ]
    ).into()
}
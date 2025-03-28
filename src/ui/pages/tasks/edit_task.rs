use iced::{
    widget::{
        button, column, container, text, Row, Space
    }, Color, Element, Length, Padding
};

use crate::{
    core::update::{tasks::TasksMsg, Message}, icons::arrow_back_icon, 
    ui::{
        components::{buttons, headers::button_n_text, inputs}, 
        Page, ToDo
    }
};

pub fn func(todo: &ToDo) -> Element<Message> {
    let mut buttons = Vec::new();
    for i in 0..5{
        buttons.push(
            container(
                button("")
                .on_press(
                    Message::Tasks(TasksMsg::PriorityEditChange(i))
                ).style(move |theme, status| ToDo::priority_button(
                    ToDo::get_theme(theme), todo.edit.priority, 
                    status, i
                )).width(30).height(30)
            ).center_x(Length::Fill).into(),
        );
    };

    container(
        column![
            button_n_text(
                arrow_back_icon(), Message::ChangePage(Page::TasksList),
                String::from("Изменить задачу")
            ),
            container(
                column![
                    container(
                        column![
                        inputs::input(
                            "Имя", todo.edit.name.clone(), 
                            |str| Message::Tasks(TasksMsg::NameEditType(str))
                        ).width(300).padding(Padding::new(10.0)),
                        inputs::editor(
                            "Описание", &todo.edit.description,
                            |action| Message::Tasks(TasksMsg::DescriptionEditType(action))
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
                        buttons::button::hoverable(
                            "Изменить", Message::Tasks(TasksMsg::EditTask)
                        ).padding(Padding::new(10.0))
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
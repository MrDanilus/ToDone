use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input, Column
    }, Color, Element, Length, Padding
};

use crate::ui::{components::tasks_list::task, Message, Page, ToDo};

pub fn func(todo: &ToDo) -> Element<Message> {
    let tasks_container;
    if todo.tasks.is_empty(){
        tasks_container = container(
            text("Задачек нету :3")
                .size(24).color(ToDo::primary_color())
        ).center_x(Length::Fill)
        .padding(Padding::from([50.0, 0.0]));
    } else{
        let mut elements = Vec::new();
        for task in &todo.tasks{
            elements.push(task::func(task).into());
        }

        tasks_container = container(scrollable(
            Column::from_vec(elements).width(Length::Fill)
        )).center_x(Length::Fill).padding(Padding::from([0.0, 50.0]));
    }

    container(
        column![
            container(
                text("Задачки")
                    .size(32)
            ).padding(Padding::top(Padding::new(0.0), 30.0))
                .center_x(Length::Fill),
            container(
                row![
                    text_input("Поиск", &todo.search_text)
                        .on_input(Message::SearchChange)
                        .size(16).width(240.0)
                        .padding(Padding::new(4.0)),
                    container(button(text("+").size(32).line_height(0.4))
                        .on_press(Message::ChangePage(Page::CreateTask))
                        .style(|_, _| button::Style{
                            background: None,
                            text_color: Color::WHITE,
                            ..Default::default()
                    }))
                ]
            ).padding(Padding::new(15.0))
                .center_x(Length::Fill),
            tasks_container
        ]
    ).into()
}
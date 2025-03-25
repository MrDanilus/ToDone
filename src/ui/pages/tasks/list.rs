use iced::{
    widget::{
        button, column, container, row, scrollable, svg, text, text_input, Column
    }, Color, Element, Length, Padding
};

use crate::{
    core::update::{tasks_list::TasksListMsg, Message}, icons::menu_icon, 
    ui::{
        components::{headers::button_n_text, tasks_list::task}, 
        styles::ToDoTheme, Page, ToDo
    }
};

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
            elements.push(task::func(
                (todo.settings.delete_confirm, todo.task_to_delete.clone()), 
                task.1
            ).into());
        }

        tasks_container = container(scrollable(
            Column::from_vec(elements).width(Length::Fill)
        )).center_x(Length::Fill).padding(Padding::from([0.0, 50.0]));
    }

    container(
        column![
            button_n_text(
                svg(svg::Handle::from_memory(menu_icon()))
                    .width(36).height(44)
                    .style(|theme, status| svg::Style { 
                        color: Some(
                            match status {
                                svg::Status::Idle => Color::from_rgb(0.4, 0.4, 0.4),
                                svg::Status::Hovered => match ToDo::get_theme(theme){
                                    ToDoTheme::Dark => Color::WHITE,
                                    ToDoTheme::Light => Color::BLACK
                                }
                            }
                        )
                    }), 
                Message::ChangePage(Page::Settings), String::from("Задачки")),
            container(
                row![
                    text_input("Поиск", &todo.search_text)
                        .on_input(
                            |str| Message::TasksList(TasksListMsg::SearchChange(str))
                        ).size(16).width(240.0)
                        .padding(Padding::new(4.0)),
                    container(button(text("+").size(32).line_height(0.4))
                        .on_press(Message::ChangePage(Page::CreateTask))
                        .style(|theme, _| button::Style{
                            background: None,
                            text_color: match ToDo::get_theme(theme) {
                                ToDoTheme::Dark => Color::WHITE,
                                ToDoTheme::Light => Color::BLACK
                            },
                            ..Default::default()
                    }))
                ]
            ).padding(Padding::new(15.0))
                .center_x(Length::Fill),
            tasks_container
        ]
    ).into()
}
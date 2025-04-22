use iced::{
    widget::{
        button, column, container, row, scrollable, text, Column
    }, Color, Element, Length, Padding
};

use crate::{
    core::update::{tasks_list::TasksListMsg, Message}, icons::menu_icon, 
    ui::{
        components::{headers::button_n_text, inputs, tasks_list::task}, 
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
        let mut tasks = todo.tasks.values().collect::<Vec<_>>();
        // Сортировка задач
        tasks.sort_by(|task1, task2| task2.priority.cmp(&task1.priority));
        tasks.sort_by_key(|task| task.completed.clone());
        // Создание элементов
        let mut elements = Vec::new();
        for task in &tasks{
            elements.push(task::func(
                todo.window.width,
                (todo.settings.delete_confirm, todo.task_to_delete.clone()), 
                &todo.subtasks_for_task, task
            ).into());
        }

        tasks_container = container(scrollable(
            Column::from_vec(elements).width(Length::Fill)
        )).center_x(Length::Fill).padding(Padding::from([0.0, 50.0]));
    }

    container(
        column![
            button_n_text(
                menu_icon(), Message::ChangePage(Page::Menu),
                String::from("Задачки")
            ),
            container(
                row![
                    inputs::input(
                        "Поиск", todo.search_text.clone(), 
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
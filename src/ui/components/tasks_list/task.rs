use std::f32::INFINITY;

use iced::{
    alignment::Horizontal, border::Radius, 
    widget::{button, column, container, row, svg, text, Column, Space}, 
    Border, Color, Length, Padding, Renderer, Theme
};

use crate::{
    core::{functions::tasks::Task, ui::truncate_text, update::{tasks_list::TasksListMsg, Message}}, 
    icons::{check_icon, cross_icon, delete_icon, edit_icon}, 
    ui::{styles::ToDoTheme, Page, ToDo}
};

pub fn func(delete_confirm: (bool, String), task: &Task) -> Column<'static, Message, Theme, Renderer>{
    let completed = task.completed.clone();
    let priority_color;
    if !task.completed{
        priority_color = match task.priority{
            0 => Color::parse("#6B7280").unwrap(),
            1 => Color::parse("#FDBA74").unwrap(),
            2 => Color::parse("#F59E0B").unwrap(),
            3 => Color::parse("#EA580C").unwrap(),
            4 => Color::parse("#EF4444").unwrap(),
            _ => Color::from_rgb(0.2, 0.2, 0.2)
        };
    } else{
        priority_color = Color::from_rgb(0.2, 0.2, 0.2)
    }

    column![
    container(row![
            container(
                button(
                    if task.completed{
                        container(
                            svg(svg::Handle::from_memory(check_icon()))
                            .width(Length::Fill).height(Length::Fill)
                        )
                    } else{
                        container(Space::new(0, 0))
                    }
                ).style(move |_, _| button::Style { 
                    border: Border{
                        radius: Radius::new(INFINITY),
                        color: priority_color,
                        width: 1.0
                    },
                    ..Default::default()
                }).padding(Padding::from(4))
                .width(30).height(30)
                .on_press(
                    Message::TasksList(TasksListMsg::CompleteTask(task.id.clone()))
                )
            ),
            column![
                text(truncate_text(&task.name, 40)).size(20)
                .style(move |theme| text::Style { 
                    color: if !completed {
                        match ToDo::get_theme(theme) {
                            ToDoTheme::Dark => Some(Color::WHITE),
                            ToDoTheme::Light => Some(Color::BLACK),
                        }
                    }
                    else {Some(Color::from_rgb(0.65, 0.65, 0.65))}
                 }),
                text(truncate_text(&task.description, 60)).size(12)
                .color(
                    if !task.completed {Color::from_rgb(0.7, 0.7, 0.7)}
                    else {Color::from_rgb(0.35, 0.35, 0.35)}
                )
            ],
            container(
                if delete_confirm.0 && delete_confirm.1 == task.id{
                    row![
                        button(
                            svg(svg::Handle::from_memory(cross_icon()))
                            .width(Length::Fill).height(Length::Fill)
                            .style(|_, _| svg::Style { color: Some(Color::from_rgb(0.9, 0.0, 0.0)) })
                        ).style(move |_, _| button::Style { 
                            ..Default::default()
                        }).padding(Padding::from(4))
                        .width(30).height(30).on_press(
                            Message::TasksList(TasksListMsg::DeleteConfirm(String::new()))
                        ),
                        button(
                            svg(svg::Handle::from_memory(check_icon()))
                            .width(Length::Fill).height(Length::Fill)
                        ).style(move |_, _| button::Style { 
                            ..Default::default()
                        }).padding(Padding::from(4))
                        .width(30).height(30).on_press(
                            Message::TasksList(TasksListMsg::DeleteTask(task.id.clone()))
                        )
                    ]
                } else{
                    row![
                        button(
                            svg(svg::Handle::from_memory(edit_icon()))
                            .width(Length::Fill).height(Length::Fill)
                            .style(|theme, _| svg::Style{
                                color: match ToDo::get_theme(theme) {
                                    ToDoTheme::Dark => Some(Color::WHITE),
                                    ToDoTheme::Light => Some(Color::BLACK)
                                }
                            })
                        ).style(move |_, _| button::Style { 
                            ..Default::default()
                        }).padding(Padding::from(4))
                        .width(30).height(30).on_press(Message::ChangePage(Page::EditTask(task.id.clone()))),
                        button(
                            svg(svg::Handle::from_memory(delete_icon()))
                            .width(Length::Fill).height(Length::Fill)
                        ).style(move |_, _| button::Style { 
                            ..Default::default()
                        }).padding(Padding::from(4))
                        .width(30).height(30).on_press(
                            if delete_confirm.0{
                                Message::TasksList(TasksListMsg::DeleteConfirm(task.id.clone()))
                            } else{
                                Message::TasksList(TasksListMsg::DeleteTask(task.id.clone()))
                            }
                        )
                    ]
                }
            ).width(Length::Fill).align_x(Horizontal::Right)
        ].spacing(10).padding(Padding::new(10.0))
    ).width(Length::Fill).padding(Padding::from([6, 0])),
    container(Space::new(Length::Fill, 1)).width(Length::Fill)
        .style(|theme| container::Style { 
            border: Border { 
                color: match ToDo::get_theme(theme) {
                    ToDoTheme::Dark => Color::from_rgb(0.1, 0.1, 0.1),
                    ToDoTheme::Light => Color::from_rgb(0.8, 0.8, 0.8)
                }, 
                width: 1.0, 
                radius: Radius::from(10)
            },
            ..Default::default()
        })
    ]
}
use std::f32::INFINITY;

use iced::{
    alignment::Horizontal, border::Radius, widget::{button, column, container, row, svg, text, Column, Space}, Border, Color, Length, Padding, Renderer, Theme
};

use crate::{core::{tasks::Task, ui::truncate_text}, icons::{check_icon, delete_icon}, ui::Message};

pub fn func(task: &Task) -> Column<'static, Message, Theme, Renderer>{
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
                .on_press(Message::CompleteTask(task.id.clone()))
            ),
            column![
                text(truncate_text(&task.name, 40)).size(20)
                .color(
                    if !task.completed {Color::WHITE}
                    else {Color::from_rgb(0.65, 0.65, 0.65)}
                ),
                text(truncate_text(&task.description, 74)).size(12)
                .color(
                    if !task.completed {Color::from_rgb(0.7, 0.7, 0.7)}
                    else {Color::from_rgb(0.35, 0.35, 0.35)}
                )
            ],
            container(
                button(
                    svg(svg::Handle::from_memory(delete_icon()))
                    .width(Length::Fill).height(Length::Fill)
                ).style(move |_, _| button::Style { 
                    ..Default::default()
                }).padding(Padding::from(4))
                .width(30).height(30).on_press(Message::DeleteTask(task.id.clone()))
            ).width(Length::Fill).align_x(Horizontal::Right)
        ].spacing(10).padding(Padding::new(10.0))
    ).width(Length::Fill).padding(Padding::from([6, 0])),
    container(Space::new(Length::Fill, 1)).width(Length::Fill)
        .style(|_| container::Style { 
            border: Border { 
                color: Color::from_rgb(0.1, 0.1, 0.1), 
                width: 1.0, 
                radius: Radius::from(10)
            },
            ..Default::default()
        })
    ]
}
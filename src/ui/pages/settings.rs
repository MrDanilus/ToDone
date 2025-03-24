use iced::{
    alignment::Vertical, widget::{column, container, pick_list, row, svg, text}, Color, Element, Padding
};

use crate::{
    icons::arrow_back_icon, 
    ui::{
        components::{elements::toggle, headers::button_n_text}, 
        styles::ToDoTheme, Message, Page, ToDo
    }
};

pub fn func(todo: &ToDo) -> Element<Message> {
    // Доступные темы
    let themes = [
        ToDoTheme::Dark,
        ToDoTheme::Light
    ];

    container(
        column![
            button_n_text(
                svg(svg::Handle::from_memory(arrow_back_icon()))
                    .width(36).height(44)
                    .style(|theme, _| ToDo::svg_icon(
                        ToDo::get_theme(theme), Color::parse("#efefef").unwrap()
                    )),
                Message::ChangePage(Page::TasksList), String::from("Настройки")),
            column![
                row![
                    text("Подтверждать удаление задач").size(18)
                    .align_y(Vertical::Center).height(36),
                    toggle::func(todo.settings.delete_confirm, Message::ChangeDeleteConfirm)
                ].spacing(2),
                row![
                    text("Тема программы").size(18)
                        .align_y(Vertical::Center).height(36),
                    pick_list(
                        themes,
                        todo.settings.theme.clone(),
                        Message::ChangeTheme
                    ).placeholder("Выберите тему")
                    .padding(Padding::new(8.0))
                ].spacing(12),
            ].spacing(8)
            .padding(Padding::from([20.0, 50.0]))
        ]
    ).into()
}
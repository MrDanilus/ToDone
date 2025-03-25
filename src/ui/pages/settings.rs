use iced::{
    alignment::Vertical, widget::{button, column, container, pick_list, row, svg, text}, Color, Element, Length, Padding
};

use crate::{
    core::update::{settings::SettingsMsg, Message}, icons::{arrow_back_icon, github_icon}, 
    ui::{
        components::{elements::toggle, headers::button_n_text}, 
        styles::ToDoTheme, Page, ToDo
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
                    toggle::func(todo.settings.delete_confirm, 
                        Message::Settings(SettingsMsg::ChangeDeleteConfirm))
                ].spacing(2),
                row![
                    text("Тема программы").size(18)
                        .align_y(Vertical::Center).height(36),
                    pick_list(
                        themes,
                        todo.settings.theme.clone(),
                        |theme| Message::Settings(SettingsMsg::ChangeTheme(theme))
                    ).placeholder("Выберите тему")
                    .padding(Padding::new(8.0))
                ].spacing(12),
                row![
                    button(
                        svg(svg::Handle::from_memory(github_icon()))
                        .width(28).height(28)
                        .style(|theme, _| ToDo::svg_icon(
                            ToDo::get_theme(theme), Color::parse("#efefef").unwrap()
                        ))
                    ).style(move |_, _| button::Style { 
                        ..Default::default()
                    }).on_press(Message::Settings(SettingsMsg::OpenGitHub))
                ].height(Length::Fill).align_y(Vertical::Bottom)
            ].height(Length::Fill).spacing(8)
            .padding(Padding::from([20.0, 50.0]))
        ]
    ).into()
}
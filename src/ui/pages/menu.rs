use iced::{
    alignment::Vertical, 
    widget::{column, container, pick_list, row, text}, 
    Element, Length, Padding
};

use crate::{
    core::update::{menu::MenuMsg, settings::SettingsMsg, Message}, icons::{arrow_back_icon, github_icon}, 
    ui::{
        components::{buttons::{self, toggle}, headers::button_n_text}, 
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
                arrow_back_icon(), Message::ChangePage(Page::TasksList),
                String::from("Меню")
            ),
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
            ].height(Length::Fill).spacing(8)
            .padding(Padding::from([20.0, 50.0])),
            row![
                buttons::button::hoverable(
                    github_icon(), Message::Menu(MenuMsg::OpenGitHub),
                    (30, 30)
                )
            ].height(Length::Fill).align_y(Vertical::Bottom)
            .padding(Padding::new(20.0))
        ]
    ).into()
}
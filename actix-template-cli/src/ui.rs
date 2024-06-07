pub fn get_cancelable_render_config(message: &'static str) -> inquire::ui::RenderConfig<'static> {
    return inquire::ui::RenderConfig::empty()
        .with_canceled_prompt_indicator(inquire::ui::Styled::new(message));
}

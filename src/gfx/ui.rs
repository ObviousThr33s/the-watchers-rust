use Constraint::{Fill, Length, Min};
use ratatui::{
    layout::{Constraint, Layout, Margin}, style::{Color, Style, Stylize}, symbols, text::Text, widgets::{Block, Paragraph}, Frame
};

use crate::utils::logger::{self, Logger};

struct UI {

}

pub(crate) fn draw_(frame: &mut Frame, log:Logger) {

	let vertical = Layout::vertical([Length(5), Min(2), Length(5)]);
	let [title_area, main_area, status_area] = vertical.areas(frame.area());
	let horizontal = Layout::horizontal([Constraint::Percentage(100)]);
	let [main_area] = horizontal.areas(main_area);

	let style:Style = Style::new().fg(Color::LightBlue).bg(Color::Black);



	let top_block = Block::bordered()
			.border_set(symbols::border::DOUBLE)
			.title("The Watchers")
			.title_style(style)
			.clone();

	let center_screen = Block::new()
		.style(style);

	let log_line:String = log.get_latest_log();
	let log_para = Paragraph::new(Text::from(log_line)).block(top_block.clone());

	frame.render_widget(log_para, title_area);
	frame.render_widget(center_screen, main_area);
	frame.render_widget(Block::bordered().title("Status Bar"), status_area);
	
	let paragraph = Paragraph::new(Text::from("bonjour"));

	frame.render_widget(paragraph, main_area.inner(Margin::new(1, 1)));
	
}
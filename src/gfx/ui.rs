use ratatui::{
	layout::{self, Constraint, Direction, Layout, Margin, Rect}, style::{Color, Style, Stylize}, symbols::line, text::Text, widgets::{self, Block, BorderType, Borders, Paragraph}, Frame
};
use crate::utils::logger::{self, Logger};
use super::{lamp::{self, Lamp}, screen};

struct UI {

}

pub(crate) fn draw_(frame: &mut Frame, log_:Logger) {
	let style:Style = Style::new().fg(Color::LightBlue).bg(Color::Black);
	let line_count:usize = 3; 
	let mut lines_combined:Vec<String> = Vec::new();

	let style:Style = Style::new().fg(Color::LightBlue).bg(Color::Black);

	let top_block = Block::bordered()
		.title("The Watchers")
		.title_style(style)
		.border_type(BorderType::Double)
		.borders(Borders::ALL);
	let middle_block = Block::new().title_bottom("*Live*");

	let bot_block = Block::bordered()
		.title("Stats")
		.title_style(style)
		.border_type(BorderType::Double)
		.borders(Borders::TOP);

	
	let layout = Layout::default()
		.direction(
			Direction::Vertical)
		.constraints(vec![
				Constraint::Percentage(10),
				Constraint::Percentage(60),
				Constraint::Percentage(20)
			]).split(frame.area());

		
	let bottom_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![
				Constraint::Percentage(70),
				Constraint::Percentage(30)
			])
			.split(layout[2]);

	

	
	//top block widgets
	let logger_ui:Paragraph = Paragraph::new(
								Text::from(log_.get_log(line_count)
												.concat()))
				.block(top_block);

	//middle block widgets
	let mut lamp:Lamp = Lamp::init(frame.area().width.into(),
								   frame.area().height.into(), 
								   lamp::CHARSETS::Charset1
								  );
	
	lamp.make_lamp();
	
	let frame_ui:Paragraph = Paragraph::new(Text::from(lamp.to_string())).block(middle_block);
	
	//bottom block widgets
	let stats:Paragraph = Paragraph::new("Stats").block(bot_block.clone());
	let invty:Paragraph = Paragraph::new("Inventory").block(bot_block);

	//frame render calls 
	///top
	frame.render_widget(logger_ui,layout[0]);
	///mid
	frame.render_widget(frame_ui, layout[1]);
	///bot
	frame.render_widget(stats, bottom_layout[0]);
	frame.render_widget(invty, bottom_layout[1]);

}
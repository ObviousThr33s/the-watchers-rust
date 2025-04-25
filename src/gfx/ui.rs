use ratatui::{
	layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::Text, widgets::{Block, BorderType, Borders, Paragraph}, Frame
};
use crate::{game::entity::{self, Entity}, utils::logger::Logger};

use super::render::{self, Render};

struct _UI {

}

fn draw_lamp<'a>(width: u16, height: u16, entity_array:&mut Vec<Entity>) -> Paragraph<'a> {

	
	let middle_block = Block::new().title_bottom("*Live*");
	
	let mut lamp: Render = Render::init(width.into(), height.into(), render::CHARSETS::Charset1);
	
	//make entity array 

	entity_array.push(entity::Entity::new(0, 0, 'a'));

	let en:entity::Entity = entity_array[0].clone();
	lamp.rasterize(vec![en.clone()]);
	
	let frame_ui: Paragraph = Paragraph::new(Text::from(lamp.to_string())).block(middle_block);

	frame_ui
}

fn draw_stats<'a> (style:Style, border:BorderType) -> Paragraph <'a>{
	let bot_block_right = Block::bordered()
		.title("Stats")
		.title_style(style)
		.border_type(border)
		.borders(Borders::LEFT);

	let stats:Paragraph = Paragraph::new("Stats").block(bot_block_right);

	stats
}

fn draw_invty<'a> (style:Style, border:BorderType) -> Paragraph <'a>{
	let bot_block_left = Block::bordered()
		.title("Inventory")
		.title_style(style)
		.border_type(border)
		.borders(Borders::RIGHT);

	let invty:Paragraph = Paragraph::new("Inventory").block(bot_block_left);
	invty
}

fn draw_log <'a> (style:Style, border:BorderType, line_count:usize, log_:Logger) -> Paragraph <'a>{
	let top_block = Block::bordered()
		.title(format!("The Watchers v{}", log_.clone().get_version()))
		.title_style(style)
		.border_type(border)
		.borders(Borders::ALL);
	
	let logger_ui:Paragraph = Paragraph::new(
		Text::from(log_.get_log(line_count)
						.concat()))
		.block(top_block);

	logger_ui
}

pub(crate) fn draw_(frame: &mut Frame, mut log_:Logger, mut entities:&mut Vec<Entity>) {
	
	log_.log("Drawing UI...".to_string().as_str());
	
	let style:Style = Style::new().fg(Color::LightBlue).bg(Color::Black);
	let border:BorderType = BorderType::Double;

	let line_count:usize = 3; 

	let layout = Layout::default()
		.direction(
			Direction::Vertical)
		.constraints(vec![
				Constraint::Percentage(10),
				Constraint::Percentage(70),
				Constraint::Percentage(20)
			]).split(frame.area());
	
	//top block widgets
	

	//middle block widgets
	
	let frame_render = draw_lamp(frame.area().width, frame.area().height, &mut entities);
	
	//bottom block widgets

	let bottom_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![
				Constraint::Percentage(70),
				Constraint::Percentage(30)
			])
			.split(layout[2]);

	//frame render calls 
	//top
	let logger_ui:Paragraph = draw_log(style, border, line_count, log_);
	frame.render_widget(logger_ui,layout[0]);
	//mid
	frame.render_widget(frame_render, layout[1]);

	//Bottom UI
	let outter_bottom:Block = Block::bordered().border_type(border).borders(Borders::TOP);

	frame.render_widget(outter_bottom.clone(), layout[2]);

	let stats:Paragraph = draw_stats(style, border);
	let invty:Paragraph = draw_invty(style, border);
	let inner_left = outter_bottom.inner(bottom_layout[0]);
	let inner_right = outter_bottom.inner(bottom_layout[1]);
	
	
	frame.render_widget(invty, inner_left);
	frame.render_widget(stats, inner_right);
	
}
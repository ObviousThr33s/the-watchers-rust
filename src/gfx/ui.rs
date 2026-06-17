use ratatui::{
	layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Block, BorderType, Borders, Clear, Paragraph}, Frame,
};
use crate::{game::spaces::field::Field, utils::logger::Logger};

use super::{charset::CHARSETS, minimap::render::Render, portal::Portal};

struct _UI {
	//Too complicated to explain one comment at a time but essentially
	//each part of the frame is split into sections and then widgets are gener
	//-ated and add to their respective part of th frame
	//each draw method has a hook to that part of the game which is being rend
	//0ered

}

fn draw_portal<'a>(screen: &'a String) -> Paragraph<'a> {
	let p: Paragraph = Paragraph::new(Text::from(screen.as_str()));
	p
}

fn draw_center<'a>(width: u16, height: u16, entity:&Field) -> Paragraph<'a> {

	let middle_block = Block::new().title_bottom("*Live*");
	
	let mut lamp: Render = Render::init(width.into(), height.into(), CHARSETS::Charset0);
	
	lamp.rasterize(entity);
	
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

fn draw_minimap<'a>(style:Style, border:BorderType, entity:&Field, player_pos:(i16, i16)) -> Paragraph<'a> {
	let bot_block_minimap = Block::bordered()
		.title("Map")
		.title_style(style)
		.border_type(border);

	// A small ASCII map (20x10) scrolled to keep the player centered.
	let map_text = entity.to_ascii_map(20, 10, player_pos.0, player_pos.1);

	let minimap:Paragraph = Paragraph::new(map_text).block(bot_block_minimap);
	minimap
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

fn draw_log <'a> (style:Style, border:BorderType, log_:&Logger) -> Paragraph <'a>{
	let top_block = Block::bordered()
		.title(format!("The Watchers v{}", log_.clone().get_version()))
		.title_style(style)
		.border_type(border)
		.borders(Borders::ALL);
	
	let logger_ui:Paragraph = Paragraph::new(
		Text::from(log_.clone().get_log()
						.concat()))
		.block(top_block);

	logger_ui
}

pub(crate) fn draw_(frame: &mut Frame, screen:&String, entities:&Field, log_:&Logger, player_pos:(i16, i16), portal:&Portal) {
	default(frame, screen, entities, log_, player_pos, portal);
}

pub(crate) fn default(frame: &mut Frame, screen:&String, entities:&Field, log_:&Logger, player_pos:(i16, i16), portal:&Portal) {
	let mut _frame_sizes: Vec<( u16, u16)> = Vec::new();

	let style:Style = Style::new().fg(Color::LightBlue).bg(Color::Black);
	let border:BorderType = BorderType::Double;


	let layout = Layout::default()
		.direction(
			Direction::Vertical)
		.constraints(vec![
				Constraint::Percentage(20),
				Constraint::Percentage(60),
				Constraint::Percentage(20)
			]).split(frame.area());
	
	//top block widgets
	

	//middle block widgets
	let frame0 = draw_center(frame.area().width, frame.area().height, entities);
	

	let frame1:Paragraph = draw_portal(screen);
	//bottom block widgets

	let bottom_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(vec![
				Constraint::Percentage(20),
				Constraint::Percentage(30),
				Constraint::Percentage(25),
				Constraint::Percentage(25)
			])
			.split(layout[2]);

	//frame render calls 
	//top
	let logger_ui:Paragraph = draw_log(style, border, log_);
	frame.render_widget(logger_ui,layout[0]);
	//mid
	frame.render_widget(frame1, layout[1]);

	// An entity reveals itself: when the portal holds art (the player can see
	// something), draw it as a bordered panel floating over the view. The frame
	// is deliberately bare here — its final look is yours to shape.
	if !portal.art.is_empty() && portal.art != "none" {
		let body = if portal.prompt.is_empty() || portal.prompt == "none" {
			portal.art.clone()
		} else {
			format!("{}\n\n{}", portal.art, portal.prompt)
		};
		let rows = body.lines().count() as u16 + 2;
		let cols = body.lines().map(|l| l.chars().count()).max().unwrap_or(0) as u16 + 2;
		let area = centered_rect(layout[1], cols, rows);
		let reveal = Paragraph::new(Text::from(body))
			.block(Block::bordered().border_type(border).title("✦"));
		frame.render_widget(Clear, area);
		frame.render_widget(reveal, area);
	}

	//Bottom UI
	let outter_bottom:Block = Block::bordered().border_type(border).borders(Borders::TOP);

	frame.render_widget(outter_bottom.clone(), layout[2]);

	let stats:Paragraph = draw_stats(style, border);
	let invty:Paragraph = draw_invty(style, border);
	let minimap:Paragraph = draw_minimap(style, border, entities, player_pos);
	let inner_left = outter_bottom.inner(bottom_layout[0]);
	let inner_cent = outter_bottom.inner(bottom_layout[1]);
	let inner_minimap = outter_bottom.inner(bottom_layout[2]);
	let inner_right = outter_bottom.inner(bottom_layout[3]);
	
	frame.render_widget(invty, inner_left);
	frame.render_widget(frame0, inner_cent);
	frame.render_widget(minimap, inner_minimap);
	frame.render_widget(stats, inner_right);
}

/// A `width` x `height` rectangle centered inside `area`, clamped to fit.
fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
	let w = width.min(area.width);
	let h = height.min(area.height);
	Rect {
		x: area.x + (area.width - w) / 2,
		y: area.y + (area.height - h) / 2,
		width: w,
		height: h,
	}
}
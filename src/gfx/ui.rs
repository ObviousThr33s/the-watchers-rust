//! The UI layout: how a frame is split into panels and what each one draws.
//! [`draw_`] is the entry point [`gfx::render`](crate::gfx::render) calls; it
//! carves the frame into a top log bar, a central first-person view, and a bottom
//! row of panels (inventory, field, map, stats), then renders a widget into each.
//! A reveal overlay floats over the view when the player is looking at something.

use ratatui::{
	layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Block, BorderType, Borders, Clear, Paragraph}, Frame,
};
use crate::{game::item::Item, game::spaces::field::Field, utils::logger::Logger};

use super::{minimap::render::Render, portal::Portal};

//The big question here has always been, how can the lifetimes be used more efficiently.

struct _UI {
	//Too complicated to explain one comment at a time but essentially
	//each part of the frame is split into sections and then widgets are gener
	//-ated and add to their respective part of th frame
	//each draw method has a hook to that part of the game which is being rend
	//0ered
}

/// The central panel's body: the first-person view as a paragraph, straight from
/// the raycast text.
fn draw_portal<'a>(screen: &'a String) -> Paragraph<'a> {
	let p: Paragraph = Paragraph::new(Text::from(screen.as_str()));
	p
}

/// The "Field" panel: a top-down rasterized view of the field, drawn into a
/// fresh [`Render`] panel and handed over as styled text so colour survives.
fn draw_center<'a>(width: u16, height: u16, entity:&Field, player_pos:(i16, i16), style: Style, border: BorderType) -> Paragraph<'a> {

	// A neat frame so this map reads as a panel of its own, matching the "Map"
	// minimap beside it — same border type and colour as the rest of the bottom UI.
	let middle_block = Block::bordered()
		.title("Field")
		.title_style(style)
		.border_type(border);

	let mut lamp: Render = Render::init(width.into(), height.into());

	// Scroll the field so the player rides the middle: the world slides past as they
	// move instead of them walking off a fixed view. That motion is the cool part —
	// the field itself tells you you're travelling.
	let origin_x = player_pos.0 - (width as i16) / 2;
	let origin_y = player_pos.1 - (height as i16) / 2;
	lamp.rasterize_at(entity, origin_x, origin_y);

	// Hand the panel over as styled text so cell colour/attributes survive to the
	// screen (a flat string would drop them).
	let frame_ui: Paragraph = Paragraph::new(lamp.to_text()).block(middle_block);

	frame_ui
}

/// The "Stats" panel: the read-out of whatever the player currently sees, or a
/// muted dash when nothing is in view.
fn draw_stats<'a>(style: Style, border: BorderType, readout: &str) -> Paragraph<'a> {
	let bot_block_right = Block::bordered()
		.title("Stats")
		.title_style(style)
		.border_type(border)
		.borders(Borders::LEFT);

	// The panel reads out whatever the player can currently see. With nothing in
	// view there's no signal, so we show a muted dash rather than stale numbers.
	let body = if readout.trim().is_empty() {
		"—".to_string()
	} else {
		readout.to_string()
	};

	Paragraph::new(Text::from(body)).block(bot_block_right)
}

/// The "Map" panel: the player-centered minimap, sized to the box it lives in.
/// Its extent comes from the resolved panel Rect — the place wills its own size,
/// no hardcoded grid — so it fills its box the way the Field panel does.
fn draw_minimap<'a>(width:u16, height:u16, style:Style, border:BorderType, entity:&Field, player_pos:(i16, i16)) -> Paragraph<'a> {
	let bot_block_minimap = Block::bordered()
		.title("Map")
		.title_style(style)
		.border_type(border);

	// The ascii map is sized to the panel's own Rect and snapped to a chunk grid:
	// it shows the chunk the player stands in and jumps a whole chunk when they
	// cross its edge. ratatui is about ratios: the box decides the extent.
	let map_text = entity.to_chunk_map(width.into(), height.into(), player_pos.0, player_pos.1);

	Paragraph::new(map_text).block(bot_block_minimap)
}

/// The "Inventory" panel: the glyph and name of each thing the player carries, or
/// a muted note when their hands are empty. A read-out only — there is no selecting
/// here (looking is the only selector), so it never grows a second control scheme.
fn draw_invty<'a>(style: Style, border: BorderType, inventory: &[Item]) -> Paragraph<'a> {
	let bot_block_left = Block::bordered()
		.title("Inventory")
		.title_style(style)
		.border_type(border)
		.borders(Borders::RIGHT);

	let body = if inventory.is_empty() {
		"— empty —".to_string()
	} else {
		inventory
			.iter()
			.map(|it| format!("{} {}", it.glyph, it.name))
			.collect::<Vec<_>>()
			.join("\n")
	};

	Paragraph::new(Text::from(body)).block(bot_block_left)
}

/// The controls overlay: a one-line HUD of the key binds, floated along the
/// bottom of the first-person view so what you can do stays on screen, not in your
/// head. Later this same overlay is where the "NPC/item ahead" indicator lands.
const CONTROLS: &str = " [wasd] move    [r] drop    [q] quit ";

fn draw_controls<'a>() -> Paragraph<'a> {
	Paragraph::new(Text::from(CONTROLS))
		.style(Style::new().fg(Color::DarkGray))
		.alignment(Alignment::Center)
}

/// The top log bar: the title carries the product name and version, the body the
/// live developer log.
fn draw_log <'a> (style:Style, border:BorderType, log_:&Logger) -> Paragraph <'a>{
	let top_block = Block::bordered()
		.title(format!("{} v{}", crate::NAME, log_.get_version()))
		.title_style(style)
		.border_type(border)
		.borders(Borders::ALL);

	let logger_ui:Paragraph = Paragraph::new(
		Text::from(log_.get_log()
						.concat()))
		.block(top_block);

	logger_ui
}

/// Render the whole frame. The entry point [`gfx::render`](crate::gfx::render)
/// calls; delegates to [`default`], the one layout for now.
pub(crate) fn draw_(frame: &mut Frame, screen:&String, entities:&Field, log_:&Logger, player_pos:(i16, i16), portal:&Portal, inventory:&[Item]) {
	default(frame, screen, entities, log_, player_pos, portal, inventory);
}

/// The default frame layout: a top log bar, the central first-person view, and
/// the bottom row of panels — plus the floating reveal overlay when the portal
/// holds art.
pub(crate) fn default(frame: &mut Frame, screen:&String, entities:&Field, log_:&Logger, player_pos:(i16, i16), portal:&Portal, inventory:&[Item]) {
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
	// The Field panel's buffer is sized to its resolved Rect below, once the
	// bottom row is laid out — ratatui is about ratios, so we rasterize exactly
	// the cells the panel will show rather than the whole frame.

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

	// Controls overlay: a one-line HUD along the bottom edge of the view. Always on
	// for now; it's the seam the NPC/item indicator will share.
	let controls_area = Rect {
		x: layout[1].x,
		y: layout[1].y + layout[1].height.saturating_sub(1),
		width: layout[1].width,
		height: 1,
	};
	frame.render_widget(Clear, controls_area);
	frame.render_widget(draw_controls(), controls_area);

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
		// Placeholder frame for the reveal — a rounded border (softer than the
		// double-line UI chrome, so it reads as an apparition) and a centered
		// mark. Swap this block for your own design whenever you like.
		let reveal = Paragraph::new(Text::from(body)).block(
			Block::bordered()
				.border_type(BorderType::Rounded)
				.title("✦")
				.title_alignment(Alignment::Center),
		);
		frame.render_widget(Clear, area);
		frame.render_widget(reveal, area);
	}

	//Bottom UI
	let outter_bottom:Block = Block::bordered().border_type(border).borders(Borders::TOP);

	frame.render_widget(outter_bottom.clone(), layout[2]);

	let stats:Paragraph = draw_stats(style, border, &portal.stats);
	let invty:Paragraph = draw_invty(style, border, inventory);
	let inner_left = outter_bottom.inner(bottom_layout[0]);
	let inner_cent = outter_bottom.inner(bottom_layout[1]);
	let inner_minimap = outter_bottom.inner(bottom_layout[2]);
	let inner_right = outter_bottom.inner(bottom_layout[3]);

	// Now that each panel's Rect is resolved, size its content to exactly that box
	// instead of the whole frame — each panel wills its own extent.
	let frame0 = draw_center(inner_cent.width, inner_cent.height, entities, player_pos, style, border);
	let minimap:Paragraph = draw_minimap(inner_minimap.width, inner_minimap.height, style, border, entities, player_pos);

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

#[cfg(test)]
mod tests {
	use super::draw_;
	use crate::game::Game;
	use crate::gfx::Viewport;
	use crate::gfx::portal::Portal;
	use crate::utils::{logger::Logger, time::Time};
	use ratatui::{backend::TestBackend, Terminal};

	#[test]
	fn game_runs_and_renders_headless() {
		// Drive the runtime end to end without a terminal: init the game, then a
		// full UI render through a headless backend. A panic or an out-of-bounds
		// anywhere fails this — which "it compiles" cannot catch. (Per-tick logic
		// will be folded in here once Game::update is wired.)
		let mut logger = Logger::new(Time::new(), "test".to_owned());
		let mut game = Game::new();
		game.init(&mut logger);

		let walls: Vec<(i16, i16)> =
			game.field.entities.values().map(|e| (e.x, e.y)).collect();
		let viewport = Viewport::new(78, 20, std::f32::consts::PI / 3.0);
		let view = viewport.render_raycasted(0.0, 0.0, 0.0, &walls);

		// Force the reveal panel on, so its overlay path renders too.
		let mut portal = Portal::new();
		portal.set_portal("◉".to_owned(), "seen".to_owned(), "HP 10".to_owned());

		let mut terminal =
			Terminal::new(TestBackend::new(80, 30)).expect("headless test terminal");
		terminal
			.draw(|frame| draw_(frame, &view, &game.field, &logger, (0,0), &portal, &game.inventory))
			.expect("headless draw");

		// Reaching here = init + a full UI render, no panics.
		assert!(game.field.get_entity_by_id(crate::game::entity::PLAYER).is_some());
	}
}
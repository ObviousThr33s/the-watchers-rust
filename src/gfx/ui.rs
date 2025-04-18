use Constraint::{Length};
use ratatui::{
    layout::{Constraint, Layout, Margin, Rect}, style::{Color, Style, Stylize}, symbols, text::Text, widgets::{block::Title, Block, BorderType, Paragraph}, Frame
};

use crate::utils::logger::{self, Logger};

use super::lamp::{self, Lamp};

struct UI {

}

pub(crate) fn draw_(frame: &mut Frame, log:Logger) {

	let title_area = Rect::new(0, 0, 80, 10);
	let main_area = Rect { x: 0, y: title_area.bottom(), width: 80, height: 20, };
	let stat_area = Rect { x: 0, y: main_area.bottom(), width:80, height:10, };
	let style:Style = Style::new().fg(Color::LightBlue).bg(Color::Black);

	let top_block = Block::bordered()
			.title("The Watchers")
			.title_style(style)
			.border_type(BorderType::Double);

	let center_screen = Block::default()
		.style(style);

	frame.render_widget(top_block, title_area);
	
	////
	let mut lamp:Lamp = Lamp::init(80,40, lamp::CHARSETS::Charset0);
	lamp.make_lamp();

	let paragraph = Text::from(lamp.to_string());
	
	frame.render_widget(paragraph,main_area);
	
	frame.render_widget(Block::bordered().title("Status Bar"), stat_area);
	
	let s:Vec<String> = Vec::new();

	let mut s:String = String::new();

	for i in 0..title_area.height{
		s.push_str(&log.clone().get_log_at(i.into()));
	}

	let Paragraph = Text::from(s);
	
	frame.render_widget(Paragraph, title_area.inner(Margin { horizontal: 1, vertical: 1 }));
}
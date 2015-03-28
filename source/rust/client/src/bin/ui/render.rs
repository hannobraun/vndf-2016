use std::io;
use std::io::prelude::*;

use client::interface::Message;
use render::{
	draw_border,
	Pos,
	ScreenBuffer,
};
use render::C;
use render::Color::{
	Green,
	Red,
};

use super::base::Status;


pub fn button(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, text: &str) -> io::Result<()> {
	let (foreground_color, background_color) = status.colors();

	write!(
		&mut buffer
			.writer(x, y)
			.foreground_color(foreground_color)
			.background_color(background_color),
		"{}",
		text,
	)
}

pub fn info_section(buffer: &mut ScreenBuffer, x: Pos, y: Pos, width: Pos, height: Pos, message: &Message) -> io::Result<()> {
	try!(draw_border(
		buffer,
		x, y,
		width, height,
	));

	{
		let status_writer = buffer.writer(x + 1, y + 1);

		let (mut status_writer, status) = match *message {
			Message::Notice(ref s) =>
				(status_writer.foreground_color(Green), s.as_ref()),
			Message::Error(ref s) =>
				(status_writer.foreground_color(Red  ), s.as_ref()),
			Message::None =>
				(status_writer, ""),
		};

		try!(write!(
			&mut status_writer,
			"{}",
			status
		));
	}

	Ok(())
}

pub fn list(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, width: Pos, height: Pos, first: usize, items: &[String]) -> io::Result<()> {
	let limit = x + width;

	let (foreground_color, background_color) = status.colors();

	let items: Vec<String> = if items.len() == 0 {
		vec!["none".to_string()]
	}
	else {
		items
			.iter()
			.map(|s| s.clone())
			.collect()
	};

	let mut iter = items
		.iter()
		.skip(first);

	for i in (0 .. height) {
		let item_length = match iter.next() {
			Some(item) => {
				try!(write!(
					&mut buffer
						.writer(x, y + i as Pos)
						.limit(limit)
						.foreground_color(foreground_color)
						.background_color(background_color),
					"{}",
					item,
				));

				item.chars().count()
			},
			None =>
				0,
		};

		for x in (x + item_length as Pos .. limit - 1) {
			try!(write!(
				&mut buffer
					.writer(x, y + i as Pos)
					.limit(limit)
					.foreground_color(foreground_color)
					.background_color(background_color),
				" ",
			));
		}
	}

	if first > 0 {
		try!(write!(
			&mut buffer.writer(limit - 1, y).limit(limit),
			"↑",
		));
	}

	if items.len() - first > height as usize {
		try!(write!(
			&mut buffer.writer(limit - 1, y + height - 1).limit(limit),
			"↓",
		));
	}

	Ok(())
}

pub fn main_section(buffer: &mut ScreenBuffer, x: Pos, y: Pos, width: Pos, height: Pos) -> io::Result<()> {
	draw_border(
		buffer,
		x, y,
		width, height,
	)
}

pub fn tab_header(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, label: &str) -> io::Result<()> {
	let (foreground_color, background_color) = status.colors();

	write!(
		&mut buffer
			.writer(x, y)
			.foreground_color(foreground_color)
			.background_color(background_color),
		"{}",
		label,
	)
}

pub fn tab_switcher(buffer: &mut ScreenBuffer, x: Pos, y: Pos) -> io::Result<()> {
	let mut c = C::new();
	c.c = '─';
	for x in (x .. buffer.width()) {
		try!(buffer.set(x, y + 1, c));
	}

	Ok(())
}

pub fn text_field(buffer: &mut ScreenBuffer, x: Pos, y: Pos, status: Status, width: Pos, text: &str) -> io::Result<()> {
	let limit = x + width;

	let (foreground_color, background_color) = status.colors();

	try!(write!(
		&mut buffer
			.writer(x, y)
			.limit(limit)
			.foreground_color(foreground_color)
			.background_color(background_color),
		"{}",
		text,
	));
	for x in (x + text.chars().count() as Pos .. limit) {
		try!(write!(
			&mut buffer
				.writer(x, y)
				.limit(limit)
				.foreground_color(foreground_color)
				.background_color(background_color),
			" ",
		));
	}

	buffer.cursor = if status == Status::Active {
		Some((1 + x + text.chars().count() as Pos, 1 + y))
	}
	else {
		None
	};

	Ok(())
}

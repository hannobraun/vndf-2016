use std::io::{
	BufReader,
	IoResult,
};

use self::buf_writer::BufWriter;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Perception {
	pub last_action: u64,
	pub broadcasts : Vec<String>,
}

impl Perception {
	pub fn decode(buffer: &[u8]) -> Option<Perception> {
		// TODO: Properly handle all errors.
		let mut reader = BufReader::new(buffer);

		let message = match reader.read_to_string() {
			Ok(message) =>
				message,
			Err(error) => {
				print!("Error converting message to string: {}\n", error);
				return None;
			},
		};

		let mut lines: Vec<&str> = message.split('\n').collect();

		let header = match lines.remove(0) {
			Some(header) =>
				header,
			None => {
				print!("Header line is missing\n");
				return None;
			},
		};

		let last_action = match from_str(header) {
			Some(last_action) =>
				last_action,
			None => {
				print!("Header is not a number\n");
				return None;
			},
		};

		let mut broadcasts = Vec::new();
		for line in lines.iter() {
			let mut splits: Vec<String> = line.splitn(1, ' ')
				.map(|s| s.to_string())
				.collect();

			if splits.len() != 2 && splits[0].as_slice() != "UPDATE" {
				continue;
			}

			let broadcast = match splits.pop() {
				Some(broadcast) =>
					broadcast,
				None => {
					print!("Invalid line, broadcast missing: {}\n", line);
					return None;
				},
			};

			broadcasts.push(broadcast);
		}

		Some(Perception {
			last_action: last_action,
			broadcasts : broadcasts,
		})
	}
}


pub struct PerceptionEnc<'r> {
	writer: BufWriter<'r>,
}

impl<'r> PerceptionEnc<'r> {
	pub fn new(buffer: &mut [u8], last_action: u64) -> PerceptionEnc {
		let mut writer = BufWriter::new(buffer);

		match write!(&mut writer, "{}\n", last_action) {
			Ok(()) =>
				(),
			Err(error) =>
				panic!("Error writing perception header: {}", error),
		}

		PerceptionEnc {
			writer: writer,
		}
	}

	pub fn update(&mut self, broadcast: &str) -> bool {
		let mut update = [0, ..512];

		let len = {
			let mut writer = BufWriter::new(&mut update);
			match write!(&mut writer, "UPDATE {}\n", broadcast) {
				Ok(())  => (),
				Err(_)  => return false,
			}

			writer.tell().unwrap_or_else(|_|
				panic!(
					"I/O operation on BufWriter that cannot possibly fail \
					still managed to fail somehow."
				)
			)
		};
		let update = update[.. len as uint];

		match self.writer.write(update) {
			Ok(()) => (),
			Err(_) => return false,
		}

		true
	}

	pub fn encode(self, buffer: &mut [u8]) -> IoResult<&[u8]> {
		let len = {
			let mut writer = BufWriter::new(buffer);
			match writer.write(self.writer.into_slice()) {
				Ok(())     => (),
				Err(error) => return Err(error),
			};

			writer.tell().unwrap_or_else(|_|
				panic!(
					"I/O operation on BufWriter that cannot possibly fail \
					still managed to fail somehow."
				)
			)
		};

		Ok(buffer[.. len as uint])
	}
}


mod buf_writer {
	// This is code from the Rust standard library. I copied itbecause I needed
	// the BufWriter::into_slice method that I implemented here.

	// TODO: Send PR to Rust project.


	use std::io::{
		mod,
		IoError,
		IoResult,
		SeekStyle,
	};
	use std::slice;


	fn combine(seek: SeekStyle, cur: uint, end: uint, offset: i64) -> IoResult<u64> {
		// compute offset as signed and clamp to prevent overflow
		let pos = match seek {
			io::SeekSet => 0,
			io::SeekEnd => end,
			io::SeekCur => cur,
		} as i64;

		if offset + pos < 0 {
			Err(IoError {
				kind: io::InvalidInput,
				desc: "invalid seek to a negative offset",
				detail: None
			})
		} else {
			Ok((offset + pos) as u64)
		}
	}


	/// Writes to a fixed-size byte slice
	///
	/// If a write will not fit in the buffer, it returns an error and does not
	/// write any data.
	///
	/// # Example
	///
	/// ```rust
	/// # #![allow(unused_must_use)]
	/// use std::io::BufWriter;
	///
	/// let mut buf = [0, ..4];
	/// {
	///     let mut w = BufWriter::new(&mut buf);
	///     w.write(&[0, 1, 2]);
	/// }
	/// assert!(buf == [0, 1, 2, 0]);
	/// ```
	pub struct BufWriter<'a> {
		buf: &'a mut [u8],
		pos: uint
	}

	impl<'a> BufWriter<'a> {
		/// Creates a new `BufWriter` which will wrap the specified buffer. The
		/// writer initially starts at position 0.
		#[inline]
		pub fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a> {
			BufWriter {
				buf: buf,
				pos: 0
			}
		}

		/// Consumes the `BufWriter`, returning the slice it was originally
		/// created with.
		#[inline]
		pub fn into_slice(self) -> &'a mut [u8] {
			self.buf
		}
	}

	impl<'a> Writer for BufWriter<'a> {
		#[inline]
		fn write(&mut self, buf: &[u8]) -> IoResult<()> {
			// return an error if the entire write does not fit in the buffer
			let cap = if self.pos >= self.buf.len() { 0 } else { self.buf.len() - self.pos };
			if buf.len() > cap {
				return Err(IoError {
					kind: io::OtherIoError,
					desc: "Trying to write past end of buffer",
					detail: None
				})
			}

			slice::bytes::copy_memory(self.buf[mut self.pos..], buf);
			self.pos += buf.len();
			Ok(())
		}
	}

	impl<'a> Seek for BufWriter<'a> {
		#[inline]
		fn tell(&self) -> IoResult<u64> { Ok(self.pos as u64) }

		#[inline]
		fn seek(&mut self, pos: i64, style: SeekStyle) -> IoResult<()> {
			let new = try!(combine(style, self.pos, self.buf.len(), pos));
			self.pos = new as uint;
			Ok(())
		}
	}
}

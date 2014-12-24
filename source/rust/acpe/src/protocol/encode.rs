use std::kinds::marker::NoCopy;

use root::MAX_PACKET_SIZE;

use self::buf_writer::BufWriter;
use super::{
	Message,
	Header,
	Part,
};


pub struct Encoder {
	buffer : [u8, ..MAX_PACKET_SIZE],
	no_copy: NoCopy,
}

impl Encoder {
	pub fn new() -> Encoder {
		Encoder {
			buffer : [0, ..MAX_PACKET_SIZE],
			no_copy: NoCopy,
		}
	}

	pub fn message<M: Message<H, P>, H: Header, P: Part>(&mut self, header: &H) -> MessageEncoder<M> {
		MessageEncoder::new(&mut self.buffer, header)
	}
}


pub struct MessageEncoder<'r, Message> {
	writer: BufWriter<'r>,
}

impl<'r, M: Message<H, P>, H: Header, P: Part> MessageEncoder<'r, M> {
	pub fn new(buffer: &'r mut [u8], header: &H) -> MessageEncoder<'r, M> {
		let mut writer = BufWriter::new(buffer);

		match header.write(&mut writer) {
			Ok(()) =>
				(),
			Err(error) =>
				panic!("Error writing message header: {}", error),
		}

		MessageEncoder {
			writer: writer,
		}
	}

	pub fn add(&mut self, part: &P) -> bool {
		let mut buffer = [0, ..MAX_PACKET_SIZE];

		let len = {
			let mut writer = BufWriter::new(&mut buffer);
			match part.write(&mut writer) {
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
		let addition = buffer[.. len as uint];

		match self.writer.write(addition) {
			Ok(()) => (),
			Err(_) => return false,
		}

		true
	}

	pub fn encode(self) -> &'r [u8] {
		let len = self.writer.tell().unwrap_or_else(|_|
			panic!(
				"I/O operation on BufWriter that cannot possibly fail still \
				managed to fail somehow."
			)
		);

		self.writer.into_slice()[.. len as uint]
	}
}


mod buf_writer {
	// This is code from the Rust standard library. I copied it because I needed
	// the BufWriter::into_slice method that I implemented here. My pull request
	// wasn't accepted because of the pending I/O reform, and BufWriter might
	// well be removed. I'll just leave this here for now.


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
		pub fn new(buf: &'a mut [u8]) -> BufWriter<'a> {
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

use std::io::{
	IoError,
	IoResult,
};
use std::kinds::marker::NoCopy;

use rustc_serialize::{
	json,
	Encodable,
};

use root::{
	HEADER,
	MAX_PACKET_SIZE,
	UPDATE,
};

use self::buf_writer::BufWriter;


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

	pub fn message<H, E>(&mut self, header: &H) -> MessageEncoder<H, E>
		where
			H: Encode,
			E: Encode,
	{
		MessageEncoder::new(&mut self.buffer, header)
	}
}


pub struct MessageEncoder<'r, H, E> {
	writer: BufWriter<'r>,
}

impl<'r, H, E> MessageEncoder<'r, H, E>
	where
		H: Encode,
		E: Encode,
{
	pub fn new(buffer: &'r mut [u8], header: &H) -> MessageEncoder<'r, H, E> {
		let mut writer = BufWriter::new(buffer);

		match write(&mut writer, HEADER, header) {
			Ok(()) =>
				(),
			Err(error) =>
				panic!("Error writing message header: {}", error),
		}

		MessageEncoder {
			writer: writer,
		}
	}

	pub fn update(&mut self, entity: &E) -> bool {
		let mut buffer = [0, ..MAX_PACKET_SIZE];

		let len = {
			let mut writer = BufWriter::new(&mut buffer);
			match write(&mut writer, UPDATE, entity) {
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


fn write<W, E>(writer: &mut W, prefix: &str, entity: &E) -> IoResult<()>
	where
		W: Writer,
		E: Encode,
{
	try!(write!(writer, "{} ", prefix));
	try!(entity.encode(writer));
	try!(write!(writer, "\n"));

	Ok(())
}


pub trait Encode {
	fn encode<W: Writer>(&self, writer: &mut W) -> IoResult<()>;
}

impl<'e, T> Encode for T where T: Encodable<json::Encoder<'e>, IoError> {
	fn encode<'a, W: Writer>(&self, writer: &'a mut W) -> IoResult<()> {
		// The API used here is inefficient, since it allocates a String for
		// each encoding. There's a more efficient, Writer-based one, but I
		// couldn't get it to work due to lifetime issues. This should be good
		// enough for now.
		write!(writer, "{}", json::encode(self))
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

use std::io::{Read, Result, Write};

#[derive(Debug)]
pub struct ReadStats<R> {
    content: R,
    bytes_through: usize,
    times_read: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> Self {
        Self {
            content: wrapped,
            bytes_through: 0,
            times_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.content
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.times_read
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read = self.content.read(buf)?;
        self.bytes_through += read;
        self.times_read += 1;
        Ok(read)
    }
}

#[derive(Debug)]
pub struct WriteStats<W> {
    content: W,
    bytes_through: usize,
    times_wrote: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> Self {
        Self {
            content: wrapped,
            bytes_through: 0,
            times_wrote: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.content
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.times_wrote
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let wrote = self.content.write(buf)?;
        self.bytes_through += wrote;
        self.times_wrote += 1;
        Ok(wrote)
    }

    fn flush(&mut self) -> Result<()> {
        self.content.flush()?;
        Ok(())
    }
}

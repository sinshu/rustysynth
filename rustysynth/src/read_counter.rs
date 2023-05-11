use std::io::Read;

pub(crate) struct ReadCounter<'a, R: Read> {
    reader: &'a mut R,
    count: usize,
}

impl<'a, R: Read> ReadCounter<'a, R> {
    pub(crate) fn new(reader: &'a mut R) -> Self {
        Self { reader, count: 0 }
    }

    pub(crate) fn bytes_read(&self) -> usize {
        self.count
    }
}

impl<'a, R: Read> Read for ReadCounter<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.count += len;
        Ok(len)
    }
}

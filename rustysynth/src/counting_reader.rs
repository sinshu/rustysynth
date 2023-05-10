use std::io::Read;

pub(crate) struct CountingReadWrapper<'a, R: Read> {
    reader: &'a mut R,
    bytes_read: usize,
}

impl<'a, R: Read> CountingReadWrapper<'a, R> {
    pub(crate) fn new(reader: &'a mut R) -> Self {
        Self {
            reader,
            bytes_read: 0,
        }
    }

    pub(crate) fn bytes_read(&self) -> usize {
        self.bytes_read
    }
}

impl<'a, R: Read> Read for CountingReadWrapper<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.bytes_read += len;
        Ok(len)
    }
}

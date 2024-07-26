use std::io::Write;

pub struct SizedWriter<T> where T: AsMut<[u8]> {
    writer: T,
    count: usize,
    overflow: bool,
}

impl <T> SizedWriter<T> where T: AsMut<[u8]> {
    pub fn new(writer: T) -> Self {
        Self {
            writer,
            count: 0,
            overflow: false,
        }
    }

    pub fn write_count(&self) -> usize {
        self.count
    }

    pub fn written_bytes(&self) -> &[u8] where T: AsRef<[u8]> {
        &self.writer.as_ref()[0..self.count]
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn overflowed(&self) -> bool {
        self.overflow
    }
}

impl <'buf> SizedWriter<&'buf mut [u8]> {
    pub fn from_borrowed(b: &'buf mut impl AsMut<[u8]>) -> Self {
        SizedWriter::new(b.as_mut())
    }
}

impl <const N: usize> SizedWriter<[u8; N]> {
    pub fn with_size() -> Self {
        let buf = [0; N];
        SizedWriter::new(buf)
    }

    pub fn from_owned(b: [u8; N]) -> Self {
        SizedWriter::new(b)
    }

    pub fn into_inner(self) -> [u8; N] {
        self.writer
    }
}

impl <T> Write for SizedWriter<T> where T: AsMut<[u8]> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let buf = self.writer.as_mut();
        let offset = self.count;

        if offset + bytes.len() > buf.len() {
            self.overflow = true;
        }

        let mut buf = &mut buf[offset..];
        let len = buf.write(bytes)?;
        self.count += len;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ref_write() -> std::io::Result<()> {
        let mut b = [0u8; 128];
        {
            let mut writer = SizedWriter::from_borrowed(&mut b);
            writer.write(&[1, 2, 3, 4])?;
            writer.write(&[5, 6, 7, 8, 9])?;
        }

        assert_eq!(&b[..9], &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        Ok(())
    }

    #[test]
    fn owned_write() -> std::io::Result<()> {
        let mut writer = SizedWriter::from_owned([0u8; 128]);
        writer.write(&[1, 2, 3, 4])?;
        writer.write(&[5, 6, 7, 8, 9])?;

        let b = writer.into_inner();

        assert_eq!(&b[..9], &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        Ok(())
    }

    #[test]
    fn clear() -> std::io::Result<()> {
        let mut b = [0u8; 128];
        {
            let mut writer = SizedWriter::from_borrowed(&mut b);
            writer.write(&[1, 2, 3, 4])?;
            writer.clear();
            writer.write(&[5, 6, 7, 8, 9])?;
        }

        assert_eq!(&b[..5], &[5, 6, 7, 8, 9]);
        Ok(())
    }

    #[test]
    fn overflow() -> std::io::Result<()> {
        let mut b = [0u8; 4];
        {
            let mut writer = SizedWriter::from_borrowed(&mut b);
            writer.write(&[1, 2, 3, 4])?;
            writer.clear();
            writer.write(&[5, 6, 7, 8, 9])?;
            writer.write(&[10, 11, 12, 13, 14])?;
            assert!(writer.overflowed())
        }
        assert_eq!(&b, &[5, 6, 7, 8]);
        Ok(())
    }

    #[test]
    fn overflow_noclear() -> std::io::Result<()> {
        let mut b = [0u8; 4];
        {
            let mut writer = SizedWriter::from_borrowed(&mut b);
            writer.write(&[1, 2, 3, 4])?;
            writer.write(&[5, 6, 7, 8, 9])?;
            writer.write(&[10, 11, 12, 13, 14])?;
            assert!(writer.overflowed())
        }
        assert_eq!(&b, &[1, 2, 3, 4]);
        Ok(())
    }
}

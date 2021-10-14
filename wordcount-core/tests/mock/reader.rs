use std::io;

pub struct Reader<'a> {
    contents: &'a [u8],
    index: usize,
}

impl<'a> io::Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.index == self.contents.len() {
            return Ok(0);
        }

        let len;

        if self.contents[self.index..].len() >= buf.len() {
            len = buf.len();
        } else {
            len = self.contents[self.index..].len();
        }

        self.contents[self.index..self.index + len]
            .iter()
            .enumerate()
            .for_each(|(i, content_byte)| {
                self.index += 1;
                buf[i] = *content_byte
            });

        Ok(len)
    }
}

impl<'a> Reader<'a> {
    pub fn new(contents: &str) -> Reader {
        Reader {
            contents: contents.as_bytes(),
            index: 0,
        }
    }
}

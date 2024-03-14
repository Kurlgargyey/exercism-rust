use std::{ io::{ Read, Result, Write }, marker::PhantomData };

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    wrapped: R,
    bytes_through: usize,
    reads: usize,
    phantom: ::std::marker::PhantomData<R>,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        let bytes_through = 0;
        let reads = 0;
        let phantom = PhantomData;
        ReadStats { wrapped, bytes_through, reads, phantom }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.wrapped.read(buf)?;
        self.reads += 1;
        self.bytes_through += result;
        Ok(result)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_through: usize,
    writes: usize,
    phantom: ::std::marker::PhantomData<W>,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        let bytes_through = 0;
        let writes = 0;
        let phantom = PhantomData;
        WriteStats {
            wrapped,
            bytes_through,
            writes,
            phantom,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.wrapped.write(buf)?;
        self.writes += 1;
        self.bytes_through += result;
        Ok(result)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}

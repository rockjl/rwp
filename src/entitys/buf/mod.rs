/*
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

const ADDITIONAL_SIZE: usize = 128;

#[derive(Debug, Clone, Default)]
pub struct DataBuf {
    pub buf: Vec<u8>,
    pub additional: Option<usize>,
    pub cursor: usize,
}

impl DataBuf {
    pub fn new(cap: usize, additional: Option<usize>) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
            additional,
            cursor: 0,
        }
    }
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn buf_to_string(&self) -> String {
        String::from_utf8_lossy(self.buf.as_ref()).to_string()
    }
    pub fn replace(&mut self, from: &str, to: &str) {
        let mut a = String::from_utf8_lossy(self.buf.as_ref()).to_string();
        a = a.replace(from, to);
        self.buf = a.into_bytes();
    }
    pub fn re_fill(&mut self, content: &str) {
        self.clear();
        self.buf = content.to_string().into_bytes();
    }
    pub fn clear(&mut self) {
        self.buf.clear();
        self.cursor = 0;
    }
    pub fn as_slice(&self) ->&[u8] {
        let len = self.len();
        &self.buf[self.cursor..len]
    }
}
impl bytes::buf::Buf for DataBuf {
    fn remaining(&self) -> usize {
        // println!("bytes::buf::Buf::remaininig()");
        self.len() - self.cursor
    }
    fn chunk(&self) -> &[u8] {
        // println!("bytes::buf::Buf::chunk()");
        self.as_slice()
    }
    fn advance(&mut self, cnt: usize) {
        // println!("bytes::buf::Buf::advance():{:#?}", cnt);
        self.cursor += cnt;
    }
}
unsafe impl bytes::buf::BufMut for DataBuf {
    fn remaining_mut(&self) -> usize {
        // println!("remaining_mut:{:#?}", usize::MAX - self.buf.len());
        usize::MAX - self.buf.len()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        // println!("advance_mut:{:?}", cnt);
        let len = self.buf.len();
        self.buf.set_len(len + cnt);
    }

    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        // println!(
        //     "chunk_mut:len:{:#?} - cap:{:#?}",
        //     self.buf.len(),
        //     self.buf.capacity()
        // );
        if self.buf.len() == self.buf.capacity() {
            match self.additional {
                Some(additional) => {
                    self.buf.reserve(additional);
                }
                None => {
                    self.buf.reserve(ADDITIONAL_SIZE);
                }
            }
        }
        self.buf.spare_capacity_mut().into()
    }
    fn put<T: bytes::Buf>(&mut self, mut src: T)
    where
        Self: Sized,
    {
        // println!("put");
        assert!(self.remaining_mut() >= src.remaining());

        while src.has_remaining() {
            let l;

            unsafe {
                let s = src.chunk();
                let d = self.chunk_mut();
                l = std::cmp::min(s.len(), d.len());

                std::ptr::copy_nonoverlapping(s.as_ptr(), d.as_mut_ptr() as *mut u8, l);
            }

            src.advance(l);
            unsafe {
                self.advance_mut(l);
            }
        }
    }
    fn put_slice(&mut self, src: &[u8]) {
        // println!("put_slice");
        let mut off = 0;

        assert!(
            self.remaining_mut() >= src.len(),
            "buffer overflow; remaining = {}; src = {}",
            self.remaining_mut(),
            src.len()
        );

        while off < src.len() {
            let cnt;

            unsafe {
                let dst = self.chunk_mut();
                cnt = std::cmp::min(dst.len(), src.len() - off);

                std::ptr::copy_nonoverlapping(src[off..].as_ptr(), dst.as_mut_ptr() as *mut u8, cnt);

                off += cnt;
            }

            unsafe {
                self.advance_mut(cnt);
            }
        }
    }
}
// impl AsyncRead for DataBuf {
//     fn poll_read(
//         self: std::pin::Pin<&mut Self>,
//         _: &mut std::task::Context<'_>,
//         buf: &mut tokio::io::ReadBuf<'_>,
//     ) -> Poll<std::io::Result<()>> {
//         let bytes_to_read = buf.remaining().min(self.buf.len() - self.cursor);
//         if bytes_to_read == 0 {
//             return Poll::Ready(Ok(()));
//         }
//         buf.put_slice(&self.buf[self.cursor..self.cursor + bytes_to_read]);
//         self.get_mut().cursor += bytes_to_read;
//         Poll::Ready(Ok(()))
//     }
// }
// impl AsyncWrite for DataBuf {
//     fn poll_write(
//         self: std::pin::Pin<&mut Self>,
//         _: &mut std::task::Context<'_>,
//         buf: &[u8],
//     ) -> std::task::Poll<Result<usize, std::io::Error>> {
//         self.get_mut().buf.extend_from_slice(buf);
//         Poll::Ready(Ok(buf.len()))
//     }

//     fn poll_flush(
//         self: std::pin::Pin<&mut Self>,
//         _: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), std::io::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn poll_shutdown(
//         self: std::pin::Pin<&mut Self>,
//         _: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), std::io::Error>> {
//         Poll::Ready(Ok(()))
//     }
// }
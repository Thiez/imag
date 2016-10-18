//
// imag - the personal information management suite for the commandline
// Copyright (C) 2016 contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::error::Error;

pub struct OnErr<I, F>{
    iter: I,
    f: F
}

impl<I, F, T, E> Iterator for OnErr<I, F> where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(&E)
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(e)) => {
                (self.f)(&e);
                Some(Err(e))
            },
            other => other
        }
    }
}

pub struct ConsumeErr<I, F> {
    iter: I,
    f: F
}

impl<I, F, T, E> Iterator for ConsumeErr<I, F> where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(E)
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
         loop {
            match self.iter.next() {
                Some(Err(e)) => (self.f)(e),
                Some(Ok(item)) => return Some(item),
                _ => return None
            }
        }
    }
}

pub trait TraceIterator<T, E> : Iterator<Item = Result<T, E>> + Sized {
    fn trace_unwrap(self) -> ConsumeErr<Self, fn(E)> where E: Error {
        fn trace_consume<E: Error>(e: E) { ::trace::trace_error(&e) }
        ConsumeErr { iter: self, f: trace_consume as fn(E) }
    }

    fn on_err<F>(self, f: F) -> OnErr<Self, F>  where F: Fn(&E) {
        OnErr { iter: self, f: f }
    }

    fn unwrap_with<F>(self, f: F) -> ConsumeErr<Self, F> where F: Fn(E) {
        ConsumeErr { iter: self, f: f }
    }
}

impl<I, T, E> TraceIterator<T, E> for I where
    I: Iterator<Item = Result<T, E>>,
    E: Error
{}


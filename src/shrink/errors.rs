
///
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    _BufferEndReached,
    _MalformedVarint7,
    _MalformedVarint32,
    _MalformedVarint64,
    _MalformedVaruint7,
    _MalformedVaruint1,
    _MalformedVaruint32,
}

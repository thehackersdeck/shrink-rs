
///
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BufferEndReached,
    MalformedVarint7,
    MalformedVarint32,
    MalformedVarint64,
    MalformedVaruint7,
    MalformedVaruint1,
    MalformedVaruint32,
}

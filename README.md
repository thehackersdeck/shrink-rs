# shrink
A Lean Data Exchange Format for the Web


### FORMAT
- Stack based

##### Preamble
The preamble is used to determine if data is little endian.

- version - 1 byte - `1`
- tag     - 1 byte - `s`

##### Body

Each value is preceded by a type.

Nested structures are enclosed in delimiters.

An array structure contains a sequence of values.

A map structure contains key-value pairs seperated key_value delimiter.

**TYPES**
- int8    - 4 bits - `0`
- int16   - 4 bits - `1`
- int32   - 4 bits - `2`
- int64   - 4 bits - `3`
- uint8   - 4 bits - `4`
- uint16  - 4 bits - `5`
- uint32  - 4 bits - `6`
- uint64  - 4 bits - `7`
- float32 - 4 bits - `8`
- float64 - 4 bits - `9`
- string  - 4 bits - `10`

**DELIMITERS**
- key_value  - 4 bits - `15`
- nest_open  - 4 bits - `14`
- nest_close - 4 bits - `13`

**VALUES**
- integers & floats - variable - LEB128 encoding
- string            - variable - UTF-8 encoding

### TODOs
- [ ] Conversion from shrink to JSON
- [ ] Conversion from JSON to shrink
- [ ] Deserializing from shrink to object
- [ ] Serializing from object to shrink


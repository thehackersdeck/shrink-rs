# shrink
A Lean Data Exchange Format for the Web


### FORMAT
- Stack based

##### Preamble
The preamble is used to determine if data is little endian.

- version - 1 byte - `1`
- tag     - 1 byte - `s`


##### Body

**TYPES**
- int8    - 1 byte - `0`
- int16   - 1 byte - `1`
- int32   - 1 byte - `2`
- int64   - 1 byte - `3`
- uint8   - 1 byte - `4`
- uint16  - 1 byte - `5`
- uint32  - 1 byte - `6`
- uint64  - 1 byte - `7`
- float32 - 1 byte - `8`
- float64 - 1 byte - `9`
- string  - 1 byte - `10`

**MISC**
- open  - 1 byte - `0`
- close - 1 byte - `0`

### TODOs
- [ ] Conversion from shrink to JSON
- [ ] Conversion from JSON to shrink
- [ ] Deserializing from shrink to object
- [ ] Serializing from object to shrink


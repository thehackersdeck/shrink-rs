# shrink
A Lean Data Exchange Format for the Web


### FORMAT
- Stack based

##### Preamble
The preamble is used to determine if data is little endian.

- version - 1 byte - `1`
- tag     - 1 byte - `s` (ASCII)

##### Body

Each value is preceded by a type.

Nested structures are enclosed in delimiters.

An array structure contains a sequence of values.

A map structure contains key-value pairs seperated key_value delimiter.

Strings are null delimited.

**TYPES**
```
- int8    - 4 bits - `0`  - `0x00`
- int16   - 4 bits - `1`  - `0x01`
- int32   - 4 bits - `2`  - `0x02`
- int64   - 4 bits - `3`  - `0x03`
- uint8   - 4 bits - `4`  - `0x04`
- uint16  - 4 bits - `5`  - `0x05`
- uint32  - 4 bits - `6`  - `0x06`
- uint64  - 4 bits - `7`  - `0x07`
- float32 - 4 bits - `8`  - `0x08`
- float64 - 4 bits - `9`  - `0x09`
- string  - 4 bits - `10` - `0x0A`
```

**DELIMITERS**
```
- key_value  - 4 bits - `15` - `0x0F`
- nest_open  - 4 bits - `14` - `0x0E`
- nest_close - 4 bits - `13` - `0x0D`
```

**VALUES**
```
- integers          - variable - LEB128 encoding
- floats            - fixed    - IEEE 754 encoding
- string            - variable - UTF-8 encoding
```

### SIMPLE EXAMPLE

```json
{
    "name": "James Bourdelon",
    "age": 42,
    "height": 170.688,
    "favorite_quotes": {
        "Walt Disney": "The way to get started is to quit talking and begin doing",
        "Anne Frank": "Whoever is happy will make others happy too"
    },
    "spoken_languages": [
        "English",
        "German"
    ]
}
```

```py
0x01 0x73 # 1 s (Preamble)
0x0F 0x0A 0x6E 0x61 0x6D 0x65 0x00 # name:
0x0A 0x4a 0x61 0x6d 0x65 0x73 0x20 0x42 0x6f 0x75 0x72 0x64 0x65 0x6c 0x6f 0x6e 0x00 # "James Bourdelon"
0x0F 0x0A 0x61 0x67 0x65 0x00 # age:
0x00 0x2A # 42
0x0F 0x0A 0x68 0x65 0x69 0x67 0x68 0x74 0x00 # height:
0x08 0x43 0x2A 0xB0 0x21 # 170.688
0x66 0x61 0x76 0x6f 0x72 0x69 0x74 0x65 0x5f 0x71 0x75 0x6f 0x74 0x65 0x73 0x00 # favorite_quotes:
0x0E # {
0x0F 0x0A 0x57 0x61 0x6c 0x74 0x20 0x44 0x69 0x73 0x6e 0x65 0x79 0x0E 0x00 # "Walt Disney":
0x0A 0x54 0x68 0x65 0x20 0x77 0x61 0x79 0x20 0x74 0x6f 0x20 0x67 0x65 0x74 0x20 0x73 0x74 0x61 0x72 0x74 0x65 0x64 0x20 0x69 0x73 0x20 0x74 0x6f 0x20 0x71 0x75 0x69 0x74 0x20 0x74 0x61 0x6c 0x6b 0x69 0x6e 0x67 0x20 0x61 0x6e 0x64 0x20 0x62 0x65 0x67 0x69 0x6e 0x20 0x64 0x6f 0x69 0x6e 0x67 0x00 # "The way to get started is to quit talking and begin doing"
0x0F 0x0A 0x41 0x6e 0x6e 0x65 0x20 0x46 0x72 0x61 0x6e 0x6b 0x00 # "Anne Frank":
0x0A 0x57 0x68 0x6f 0x65 0x76 0x65 0x72 0x20 0x69 0x73 0x20 0x68 0x61 0x70 0x70 0x79 0x20 0x77 0x69 0x6c 0x6c 0x20 0x6d 0x61 0x6b 0x65 0x20 0x6f 0x74 0x68 0x65 0x72 0x73 0x20 0x68 0x61 0x70 0x70 0x79 0x20 0x74 0x6f 0x6f 0x00 # "Whoever is happy will make others happy too"
0x0D # }
0x0A 0x73 0x70 0x6f 0x6b 0x65 0x6e 0x5f 0x6c 0x61 0x6e 0x67 0x75 0x61 0x67 0x65 0x73 0x00 # spoken_languages
0x0E # [
0x0A 0x45 0x6e 0x67 0x6c 0x69 0x73 0x68 0x00 # "English"
0x0A 0x47 0x65 0x72 0x6d 0x61 0x6e 0x00 # "German"
0x0D # ]
```

For this contrived example, the JSON version takes 325 bytes while Shrink takes 235.

### TODOs
- [ ] Conversion from shrink to JSON
- [ ] Conversion from JSON to shrink
- [ ] Deserializing from shrink to object
- [ ] Serializing from object to shrink

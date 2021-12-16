# Common

## Message Format

### A

- Opcode
- Some fields

Pros:
- Easy

Cons:
- New formats must be added simultanuouly at both sides
- Messages kinds are limited by the Opcode size

### B

- Opcode
- Length
- Some fields

Pros:
- Easy
- Can add new packets of different lengths before the other side supports them
- Easy to accumulate bytes before we should attempt to parse them

Cons:
- Messages kinds are limited by the Opcode size

### C

- Length
- Payload (any serialization format)
  - Some fields

Pros:
- Easy
- Can add new packets of different lengths before the other side supports them
- Easy to accumulate bytes before we should attempt to parse them
- No Opcode size limitation

Cons:
- The serialization format might have already preserved the length information, so it's duplicated

#### D

- Any serialization format
  - Other fields

Pros:
- Can add new packets of different lengths before the other side supports them
- No Opcode size limitation
- No length duplication

Cons:
- Hard
- Hard to accumulate bytes before we should attempt to parse them, because the deserializer implementation doesn't support incremental deserialization

## Terminal Access System

- auth: name+pass
- ls
- cd
- who
- kill (admin)
- logout
- client per allowed command mappings

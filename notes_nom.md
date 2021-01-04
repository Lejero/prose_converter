# Nom Notes

## Uncategorized
- What is big and little endianness?

## Useful Parsers
- ws! was deptecated
- i16, i32, i62, i128, u16, u32, u64, u128 macros all in nom::* but functions for f32, f64 are in nom::number::complete::* or nom::number::streaming::* along with 8 bit variants for unsigned and signed integers and little and big endian parsers
- Numbers:
  - there are both f32, f64 and float, double variants. The float and double variants seem to use an external crate while the doc says the 'f' variants handle little and big endianness.
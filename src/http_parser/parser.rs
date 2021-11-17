use nom::bytes::complete::take;
use nom::character::*;
use nom::number::complete::be_u16;
use nom::sequence::tuple;
use nom::IResult;
use nom::*;

fn is_version(c: u8) -> bool {
    c >= b'0' && c <= b'9' || c == b'.'
}

// combine all previous parsers in one function
// pub fn request_line(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8], &[u8])> {
//     // tuple takes as argument a tuple of parsers and will return
//     // a tuple of their results
//     let (input, (method, _, url, _, version, _)) =
//         tuple((method, space, url, space, http_version, line_ending))(i)?;

//     Ok((input, (method, url, version)))
// }

use nom::IResult;

pub fn parse_schemix_file(input: &str) -> IResult<&str, ()> {
    // TODO: Implement parser for .smx files
    Ok((input, ()))
}
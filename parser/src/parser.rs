mod and_then;
mod attempt;
mod between;
mod flat_map;
mod followed_by;
mod from_fn;
mod from_str;
mod map;
mod num;
mod optional;
mod or;
mod recognize;
mod satisfy;
mod satisfy_map;
mod tokens;

use super::*;

pub use from_fn::{from_fn, from_fn_mut, from_fn_once};
pub use num::*;
pub use satisfy::satisfy;
pub use satisfy_map::{satisfy_map, satisfy_map_mut, satisfy_map_once};
pub use tokens::tokens;

pub trait ParserOnce<Input>: Sized {
    type Output;

    fn parse_once(self, input: &mut Input) -> Option<Self::Output>;

    fn parse_once_and_check_consumed(self, input: &mut Input) -> (Option<Self::Output>, bool)
    where
        Input: Stream,
    {
        let position = input.position();
        let output = self.parse_once(input);
        (output, input.position() != position)
    }

    fn parse_partial(self, mut input: Input) -> Option<Self::Output> {
        self.parse_once(&mut input)
    }

    fn parse_to_end(self, input: Input) -> Option<Self::Output>
    where
        Input: Stream,
    {
        chain((self, eof())).map(|(o, ())| o).parse_partial(input)
    }

    // TODO: require `F: FnOnce(Self::Output) -> O`
    fn map<F>(self, f: F) -> map::Map<Self, F> {
        map::map(self, f)
    }

    // TODO: require `P: ParserOnce<Input = Self::Input>` and `F: FnOnce(Self::Output) -> P`
    fn flat_map<F>(self, f: F) -> flat_map::FlatMap<Self, F> {
        flat_map::flat_map(self, f)
    }

    // TODO: require `F: FnOnce(Self::Output) -> Option<O>`
    fn and_then<F>(self, f: F) -> and_then::AndThen<Self, F> {
        and_then::and_then(self, f)
    }

    fn optional(self) -> optional::Optional<Self> {
        optional::optional(self)
    }

    fn recognize(self) -> recognize::Recognize<Self> {
        recognize::recognize(self)
    }

    fn from_str<'a, O>(self) -> from_str::FromStr<Self, O>
    where
        Self: ParserOnce<Input, Output = &'a str>,
    {
        from_str::from_str(self)
    }

    fn attempt(self) -> attempt::Attempt<Self> {
        attempt::attempt(self)
    }

    fn between<L, R>(self, left: L, right: R) -> between::Between<Self, L, R>
    where
        L: Parser<Input>,
        R: Parser<Input>,
    {
        between::between(self, left, right)
    }

    fn or<P>(self, parser: P) -> or::Or<Self, P>
    where
        P: ParserOnce<Input, Output = Self::Output>,
    {
        or::or(self, parser)
    }

    fn followed_by<P>(self, parser: P) -> followed_by::FollowedBy<Self, P>
    where
        P: ParserOnce<Input>,
    {
        followed_by::followed_by(self, parser)
    }
}

pub trait ParserMut<Input>: ParserOnce<Input> {
    fn parse_mut(&mut self, input: &mut Input) -> Option<Self::Output>;

    fn parse_mut_and_check_consumed(&mut self, input: &mut Input) -> (Option<Self::Output>, bool)
    where
        Input: Stream,
    {
        let position = input.position();
        let output = self.parse_mut(input);
        (output, input.position() != position)
    }

    // TODO: remove in favor of `many`
    fn many_mut<F, O>(self, f: F) -> many::Many<Self, F>
    where
        F: FnMut(&mut many::Iter<Self, &mut Input>) -> Option<O>,
    {
        many::many(self, f)
    }

    fn iter_many(self, input: Input) -> many::ManyIter<Self, Input> {
        many::iter(self, input)
    }

    fn skip_many(self) -> many::SkipMany<Self> {
        many::skip_many(self)
    }

    fn collect_many<I>(self) -> many::CollectMany<Self, I>
    where
        I: FromIterator<Self::Output>,
    {
        many::collect_many(self)
    }

    // TODO: remove in favor of `many1`
    fn many1_mut<F, O>(self, f: F) -> many1::Many1<Self, F>
    where
        F: FnMut(&mut many1::Iter<Self, &mut Input, Self::Output>) -> Option<O>,
    {
        many1::many1(self, f)
    }

    fn skip_many1(self) -> many1::SkipMany1<Self> {
        many1::skip_many1(self)
    }

    fn collect_many1<I>(self) -> many1::CollectMany1<Self, I>
    where
        I: FromIterator<Self::Output>,
    {
        many1::collect_many1(self)
    }

    fn sep_by_mut<P, F, O>(self, separator: P, f: F) -> sep_by::SepBy<Self, P, F>
    where
        P: ParserMut<Input>,
        F: FnMut(&mut sep_by::Iter<Self, P, &mut Input>) -> Option<O>,
    {
        sep_by::sep_by(self, separator, f)
    }

    fn iter_sep_by<P>(self, separator: P, input: Input) -> sep_by::SepByIter<Self, P, Input>
    where
        P: Parser<Input>,
    {
        sep_by::iter(self, separator, input)
    }

    fn skip_sep_by<P>(self, separator: P) -> sep_by::SkipSepBy<Self, P>
    where
        P: Parser<Input>,
    {
        sep_by::skip_sep_by(self, separator)
    }

    fn collect_sep_by<P, I>(self, separator: P) -> sep_by::CollectSepBy<Self, P, I>
    where
        P: Parser<Input>,
        I: FromIterator<Self::Output>,
    {
        sep_by::collect_sep_by(self, separator)
    }

    fn by_mut_ref(&mut self) -> &mut Self {
        self
    }
}

pub trait Parser<Input>: ParserMut<Input> {
    fn parse(&self, input: &mut Input) -> Option<Self::Output>;

    fn parse_and_check_consumed(&self, input: &mut Input) -> (Option<Self::Output>, bool)
    where
        Input: Stream,
    {
        let position = input.position();
        let output = self.parse(input);
        (output, input.position() != position)
    }

    fn many<F, O>(self, f: F) -> many::Many<Self, F>
    where
        F: Fn(&mut many::Iter<Self, &mut Input>) -> Option<O>,
    {
        many::many(self, f)
    }

    fn many1<F, O>(self, f: F) -> many1::Many1<Self, F>
    where
        F: Fn(&mut many1::Iter<Self, &mut Input, Self::Output>) -> Option<O>,
    {
        many1::many1(self, f)
    }

    fn sep_by<P, F, O>(self, separator: P, f: F) -> sep_by::SepBy<Self, P, F>
    where
        P: Parser<Input>,
        F: Fn(&mut sep_by::Iter<Self, P, &mut Input>) -> Option<O>,
    {
        sep_by::sep_by(self, separator, f)
    }

    fn by_ref(&self) -> &Self {
        self
    }
}

impl<P, I> ParserOnce<I> for &mut P
where
    P: ParserMut<I>,
{
    type Output = P::Output;

    fn parse_once(self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, I> ParserMut<I> for &mut P
where
    P: ParserMut<I>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        (*self).parse_mut(input)
    }
}

impl<P, I> ParserOnce<I> for &P
where
    P: Parser<I>,
{
    type Output = P::Output;

    fn parse_once(mut self, input: &mut I) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<P, I> ParserMut<I> for &P
where
    P: Parser<I>,
{
    fn parse_mut(&mut self, input: &mut I) -> Option<Self::Output> {
        self.parse(input)
    }
}

impl<P, I> Parser<I> for &P
where
    P: Parser<I>,
{
    fn parse(&self, input: &mut I) -> Option<Self::Output> {
        (*self).parse(input)
    }
}

pub fn token<I>(token: I::Item) -> impl Parser<I, Output = I::Item> + Copy
where
    I: Stream,
    I::Item: PartialEq,
{
    satisfy(move |t| t == token)
}

pub fn value<I, O>(output: O) -> impl Parser<I, Output = O> + Copy
where
    I: Stream,
    O: Copy,
{
    from_fn(move |_| Some(output))
}

pub fn any<I>() -> impl Parser<I, Output = I::Item> + Copy
where
    I: Stream,
{
    satisfy_map(Some)
}

pub fn eof<I>() -> impl Parser<I, Output = ()> + Copy
where
    I: Stream,
{
    from_fn(|input: &mut I| if input.is_empty() { Some(()) } else { None })
}

#[derive(Copy, Clone)]
struct String<'a>(&'a str);

impl<'a> ParserOnce<&'a str> for String<'a> {
    type Output = ();

    fn parse_once(mut self, input: &mut &'a str) -> Option<Self::Output> {
        self.parse_mut(input)
    }
}

impl<'a> ParserMut<&'a str> for String<'a> {
    fn parse_mut(&mut self, input: &mut &'a str) -> Option<Self::Output> {
        self.parse(input)
    }
}

impl<'a> Parser<&'a str> for String<'a> {
    fn parse(&self, input: &mut &'a str) -> Option<Self::Output> {
        tokens(self.0.chars()).parse_once(input)
    }
}

pub fn string(string: &str) -> impl Parser<&str, Output = ()> + Copy {
    String(string)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        let mut input = "abcde";
        assert_eq!(string("abc").recognize().parse(&mut input), Some("abc"));
        assert_eq!(input, "de");

        let mut input = "abde";
        assert_eq!(string("abc").recognize().parse(&mut input), None);
        assert_eq!(input, "de");
    }
}

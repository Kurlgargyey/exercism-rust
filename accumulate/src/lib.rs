/// What should the type of _function be?
pub fn map<I, O>(input: Vec<I>, mut function: impl FnMut(I) -> O) -> Vec<O> {
    input
        .into_iter()
        .fold(Vec::<O>::new(), |mut output, element| {
            output.push(function(element));
            output
        })
}

/// What should the type of _function be?
pub fn map<I, O>(input: Vec<I>, mut function: impl FnMut(I) -> O) -> Vec<O> {
    let mut result = Vec::<O>::new();
    for i in input {
        result.push(function(i))
    }
    result
}

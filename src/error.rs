pub enum CommandResult<T, E> {
    Ok(T),
    Err(E),
    Success,
    None,
}
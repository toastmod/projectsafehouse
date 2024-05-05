pub unsafe fn cast_bytes<T>(data: &[T]) -> &[u8] {
        core::slice::from_raw_parts::<u8>(
            data.as_ptr().cast(),
        std::mem::size_of::<T>()*data.len()
    )
}

#[cfg(test)]
mod tests {

}

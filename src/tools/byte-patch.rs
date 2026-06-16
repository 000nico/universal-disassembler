fn patch_bytes(original: &[u8], changes: Vec<byte_changes>) {
    for change in changes {
        original[changes.address] = change.new;
    }
}

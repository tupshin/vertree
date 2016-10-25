error_chain! {
    errors {
        AlreadyExists(path: String) {
            description("path already exists")
            display("path already exists: '{}'", path)
        }
        InvalidPathContent(path: String) {
            description("path does not end in a directory")
            display("path does not end in a directory: '{}'", path)
        }
        BadPath(msg: String) {
            description("path improperly formatted: must contain a leading slash and at least one
                         component")
            display("path improperly formatted: '{}'", msg)
        }
    }
}
error_chain! {
    foreign_links {
        Configuration(::bdrck_config::error::Error);
        Encoding(::data_encoding::decode::Error);
        Git(::git2::Error);
        Io(::std::io::Error);
        Json(::serde_json::Error);
        Log(::log::SetLoggerError);
        Utf8(::std::string::FromUtf8Error);
    }
}

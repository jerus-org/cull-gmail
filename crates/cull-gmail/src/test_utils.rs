use log::LevelFilter;

pub(crate) fn get_test_logger() {
    let mut builder = env_logger::Builder::new();
    builder.filter(None, LevelFilter::Debug);
    builder.format_timestamp_secs().format_module_path(false);
    let _ = builder.try_init();
}

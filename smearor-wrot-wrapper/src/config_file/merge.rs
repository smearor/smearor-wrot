pub trait MergeWithConfigFile<T> {
    /// Merges the configuration file of type T with the given command line arguments.
    fn merge_with_config_file(mut self, config: &T) -> Self;
}

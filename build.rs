use vergen::Config;

fn main() {
    let mut config = Config::default();

    *config.build_mut().timestamp_mut() = false;

    *config.git_mut().semver_mut() = false;
    *config.git_mut().commit_timestamp_mut() = false;

    *config.sysinfo_mut().enabled_mut() = false;

    vergen::vergen(config).unwrap();
}

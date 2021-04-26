#[derive(Copy, Clone, Debug)]
pub struct BuildInfo {
    pub version: &'static str,
    pub git_branch: &'static str,
    pub git_commit: &'static str,
    pub rustc_channel: &'static str,
    pub rustc_commit_date: &'static str,
    pub rustc_commit_hash: &'static str,
    pub rustc_host_triple: &'static str,
    pub rustc_llvm_version: &'static str,
    pub rustc_version: &'static str,
    pub cargo_features: &'static str,
    pub cargo_profile: &'static str,
    pub cargo_target_triple: &'static str,
}

const fn construct_buildinfo() -> BuildInfo {
    BuildInfo {
        version: env!("VERGEN_BUILD_SEMVER"),
        git_branch: env!("VERGEN_GIT_BRANCH"),
        git_commit: env!("VERGEN_GIT_SHA"),
        rustc_channel: env!("VERGEN_RUSTC_CHANNEL"),
        rustc_commit_date: env!("VERGEN_RUSTC_COMMIT_DATE"),
        rustc_commit_hash: env!("VERGEN_RUSTC_COMMIT_HASH"),
        rustc_host_triple: env!("VERGEN_RUSTC_HOST_TRIPLE"),
        rustc_llvm_version: env!("VERGEN_RUSTC_LLVM_VERSION"),
        rustc_version: env!("VERGEN_RUSTC_SEMVER"),
        cargo_features: env!("VERGEN_CARGO_FEATURES"),
        cargo_profile: env!("VERGEN_CARGO_PROFILE"),
        cargo_target_triple: env!("VERGEN_CARGO_TARGET_TRIPLE"),
    }
}

pub static BUILD_INFO: BuildInfo = construct_buildinfo();

const platform = process.env.GITLAB_CI
  ? ["@semantic-release/gitlab"]
  : ["@semantic-release/github"];

module.exports = {
  branches: ["main"],
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    [
      "@semantic-release/exec",
      {
        prepareCmd:
          "sed -i 's/^version = \".*\"/version = \"${nextRelease.version}\"/' Cargo.toml && cargo update -w && sed -i 's|v${lastRelease.version}|v${nextRelease.version}|g' README.md",
      },
    ],
    [
      "@semantic-release/git",
      {
        assets: ["Cargo.toml", "Cargo.lock", "README.md"],
        message: "chore(release): bump version to v${nextRelease.version}",
      },
    ],
    platform,
  ],
};

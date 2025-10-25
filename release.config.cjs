/**
 * @type {import('semantic-release').GlobalConfig}
 */
module.exports = {
    branches: [
        "main",
        { name: "pre", prerelease: true },
    ],
    plugins: [
        "@semantic-release/commit-analyzer",
        "@semantic-release/release-notes-generator",
        "@semantic-release/changelog",
        [
            "semantic-release-cargo",
            {
                "allFeatures": false,
                "check": true,
                "publish": false,
                "alwaysVerifyToken": false,
            }
        ],
        [
            "@semantic-release/git",
            {
                "assets": ["CHANGELOG.md", "Cargo.toml", "Cargo.lock"],
            }
        ],
    ]
};

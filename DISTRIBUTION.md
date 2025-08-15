# Distribution Guide

This document explains how to set up and distribute the `lx-rs` CLI tool across different platforms and package managers.

## 📦 Distribution Methods

### 1. Crates.io (Primary)

**Advantages:**
- ✅ Standard Rust distribution method
- ✅ Automatic versioning and dependency management
- ✅ Users can install with `cargo install`
- ✅ No need for pre-built binaries

**Setup:**
```bash
# 1. Create crates.io account at https://crates.io/
# 2. Get API token from https://crates.io/me
# 3. Login
cargo login YOUR_API_TOKEN

# 4. Publish
cargo publish
```

**User installation:**
```bash
cargo install langextract-rust --features cli
```

### 2. GitHub Releases (Automated)

**Advantages:**
- ✅ No Rust installation required for users
- ✅ Fast downloads
- ✅ Support for multiple platforms
- ✅ Automated via GitHub Actions

**Setup:**
1. The included `.github/workflows/release.yml` automatically builds binaries
2. Create a release tag: `git tag v0.1.0 && git push --tags`
3. GitHub Actions will build and publish binaries for:
   - Linux (x86_64, musl)
   - macOS (x86_64, ARM64)
   - Windows (x86_64)

**Required secrets in GitHub repository:**
- `CRATES_TOKEN`: Your crates.io API token (for auto-publishing)

**User installation:**
```bash
# Via install script (auto-detects platform)
curl -fsSL https://raw.githubusercontent.com/modularflow/langextract-rust/main/install.sh | bash -s -- --prebuilt

# Or manual download from releases page
```

### 3. Homebrew (macOS/Linux)

**Advantages:**
- ✅ Popular on macOS
- ✅ Easy updates
- ✅ Dependency management

**Setup:**
1. Create a homebrew tap repository: `homebrew-tap`
2. Add the formula file (included: `homebrew-formula.rb`)
3. Update SHA256 hashes after each release

**User installation:**
```bash
brew tap modularflow/tap
brew install lx-rs
```

### 4. Package Managers

#### Arch Linux (AUR)
Create a PKGBUILD file for the Arch User Repository.

#### Ubuntu/Debian
Create `.deb` packages using `cargo-deb`:
```bash
cargo install cargo-deb
cargo deb --features cli
```

#### RPM-based (Fedora/CentOS)
Create `.rpm` packages using `cargo-rpm`:
```bash
cargo install cargo-rpm
cargo rpm build --features cli
```

## 🚀 Release Process

### Automated Release (Recommended)

1. **Update version** in `Cargo.toml`
2. **Commit changes**: `git commit -am "Release v0.1.1"`
3. **Create tag**: `git tag v0.1.1`
4. **Push**: `git push && git push --tags`
5. **GitHub Actions will automatically:**
   - Build binaries for all platforms
   - Create GitHub release
   - Publish to crates.io

### Manual Release

```bash
# 1. Update version
vim Cargo.toml

# 2. Build and test
cargo build --features cli
cargo test

# 3. Publish to crates.io
cargo publish

# 4. Create GitHub release with binaries
# (Use the GitHub web interface or gh CLI)
```

## 📋 Platform Support

| Platform | Architecture | Status | Installation Method |
|----------|-------------|--------|-------------------|
| Linux | x86_64 | ✅ Supported | crates.io, GitHub releases, package managers |
| Linux | ARM64 | 🔄 Planned | crates.io |
| macOS | x86_64 | ✅ Supported | crates.io, GitHub releases, Homebrew |
| macOS | ARM64 (M1/M2) | ✅ Supported | crates.io, GitHub releases, Homebrew |
| Windows | x86_64 | ✅ Supported | crates.io, GitHub releases |
| Windows | ARM64 | 🔄 Planned | crates.io |

## 🔧 Setting Up Distribution

### First-time Setup

1. **Configure crates.io:**
   ```bash
   cargo login YOUR_TOKEN
   ```

2. **Set up GitHub secrets:**
   - Go to repository Settings → Secrets
   - Add `CRATES_TOKEN` with your crates.io API token

3. **Test the release process:**
   ```bash
   git tag v0.1.0-test
   git push --tags
   # Check GitHub Actions for any issues
   git tag -d v0.1.0-test
   git push --delete origin v0.1.0-test
   ```

### Updating Distribution

1. **Version bump:**
   ```bash
   # Update Cargo.toml version
   sed -i 's/version = "0.1.0"/version = "0.1.1"/' Cargo.toml
   ```

2. **Test changes:**
   ```bash
   cargo check --features cli
   cargo build --features cli
   ```

3. **Release:**
   ```bash
   git commit -am "Bump version to 0.1.1"
   git tag v0.1.1
   git push && git push --tags
   ```

## 📊 Distribution Analytics

Monitor distribution success through:

- **Crates.io stats**: Download counts and versions
- **GitHub releases**: Download statistics
- **Homebrew analytics**: Usage metrics (if using analytics)

## 🔍 Troubleshooting

### Common Issues

1. **Binary not found after installation**
   - Ensure `~/.cargo/bin` is in PATH
   - Restart terminal after installation

2. **GitHub Actions failing**
   - Check for missing secrets
   - Verify workflow file syntax
   - Check target compatibility

3. **Crates.io publish failing**
   - Verify all required fields in Cargo.toml
   - Check for version conflicts
   - Ensure API token is valid

### Platform-specific Issues

**Linux:**
- Missing dependencies for static linking
- Glibc compatibility issues

**macOS:**
- Code signing requirements
- Notarization for distribution

**Windows:**
- Antivirus false positives
- PowerShell execution policy restrictions

## 📈 Best Practices

1. **Semantic versioning**: Follow semver for version numbers
2. **Changelog**: Maintain CHANGELOG.md for release notes
3. **Testing**: Test on all target platforms before release
4. **Documentation**: Keep installation instructions updated
5. **Dependencies**: Minimize external dependencies for smaller binaries
6. **Security**: Sign releases where possible
7. **Automation**: Use CI/CD for consistent releases

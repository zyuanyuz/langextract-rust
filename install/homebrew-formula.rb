# This is a template for a Homebrew formula
# To use: create a tap repository and add this formula

class LxRs < Formula
  desc "Extract structured information from text using Large Language Models"
  homepage "https://github.com/modularflow/langextract-rust"
  version "0.1.0"
  
  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/modularflow/langextract-rust/releases/download/v#{version}/lx-rs-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_OF_ARM64_MACOS_BINARY"
    else
      url "https://github.com/modularflow/langextract-rust/releases/download/v#{version}/lx-rs-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_OF_X86_64_MACOS_BINARY"
    end
  elsif OS.linux?
    url "https://github.com/modularflow/langextract-rust/releases/download/v#{version}/lx-rs-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256_OF_LINUX_BINARY"
  end

  def install
    bin.install "lx-rs"
  end

  test do
    system "#{bin}/lx-rs", "--version"
  end
end

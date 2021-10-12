class Sdoc < Formula
 version '0.8.11'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "0b8d5294d47d58dfac9e174922b4f94770fd6849504a017ceac4c0aa3f552fdb"
  end

  def install
    bin.install "sdoc"
  end
end

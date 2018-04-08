class Sdoc < Formula
 version '0.5.0'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "a2c0abeb96b0aa9239f90500c65a2db907740c2b7ffbe1b220b1f8dcebe83dad"
  end

  def install
    bin.install "sdoc"
  end
end

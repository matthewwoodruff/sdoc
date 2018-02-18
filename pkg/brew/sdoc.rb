class Sdoc < Formula
version '0.4.0'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "12efbe89ef0f03bbc7ba05909c6ab0b8dd8c7a99c5293788bc9402059af54abd"
  end

  def install
    bin.install "sdoc"
  end
end

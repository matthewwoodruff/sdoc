class Sdoc < Formula
 version '0.8.9'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "164664fc157303a9de6ba4edf8f74c21bace20d751ec081241930978a550eef8"
  end

  def install
    bin.install "sdoc"
  end
end

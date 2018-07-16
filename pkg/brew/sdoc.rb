class Sdoc < Formula
 version '0.7.1'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "f3ea905a43a73d2b87c76a4899d6a3ef21c3715a5def1d9098f3ea41159e2336"
  end

  def install
    bin.install "sdoc"
  end
end

class Sdoc < Formula
 version '0.8.3'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "d3be31688b4645e28e68ec413d760098fd2aedf0ddf5d26a3f1f3bcdd69eea80"
  end

  def install
    bin.install "sdoc"
  end
end

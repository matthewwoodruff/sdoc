class Sdoc < Formula
 version '0.8.10'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "0703ead0de8bd367a80d9ea05bc48762acb0f739b6e4aaf84366c7cfd58ae0b9"
  end

  def install
    bin.install "sdoc"
  end
end

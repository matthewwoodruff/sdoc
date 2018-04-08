class Sdoc < Formula
 version '0.4.3'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "8b161cf1403fe6aa04370476a4fcbd87d7d6bd40561f06d752f2f3e1eb035470"
  end

  def install
    bin.install "sdoc"
  end
end

class Sdoc < Formula
 version '0.8.1'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "6533a9a4b61be8208e01e4c3cc996f3ac5bef7b14a251920939ca5e69868ad85"
  end

  def install
    bin.install "sdoc"
  end
end

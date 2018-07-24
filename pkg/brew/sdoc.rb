class Sdoc < Formula
 version '0.8.1'
  desc "Framework for building custom CLIs around shell, scripts, and executables"
  homepage "https://github.com/matthewwoodruff/sdoc"

  if OS.mac?
      url "https://github.com/matthewwoodruff/sdoc/releases/download/v#{version}/sdoc-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "89167ed4e9501b6a5d923fa460ceaf39349dda83046d2db9897a26d411f4b95f"
  end

  def install
    bin.install "sdoc"
  end
end

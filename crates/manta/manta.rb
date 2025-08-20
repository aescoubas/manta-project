class Manta < Formula
  desc "Manta CLI"
  homepage "https://github.com/eth-cscs/manta"
  url "https://github.com/eth-cscs/manta/releases/download/v{{ VERSION }}/manta-aarch64-apple-darwin.tar.xz"
  sha256 "{{ SHA }}"  
  version "{{ VERSION }}"

  def install
    bin.install "manta" 
  end
end


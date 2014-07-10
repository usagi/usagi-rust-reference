## GitHubリポジトリーから最新のRustをAndroid-NDKサポート付きで準備

### 手順

Ubuntu-14.04(x86_64)環境を例とした、
Android-NDK対応付きの
Rust最新版をビルド・インストールして準備する手順。

1. GitHub から clone
    - `git clone https://github.com/rust-lang/rust.git`
1. Android NDK まわり ( 不要なら手順を飛ばして良い )
    1. Android NDK に必要な x86 版のライブラリーパッケージを導入
        - `apt-get install libc6-i386 lib32z1 lib32stdc++6`
    1. Android NDK を入手
        - `wget -c http://dl.google.com/android/ndk/android-ndk-r9b-linux-x86_64.tar.bz2`
    1. Android NDK を展開
        - `tar xf android-ndk-r9b-linux-x86_64.tar.bz2`
    1. Android NDK standalone toolchain をセットアップ
        - `android-ndk-r9b/build/tools/make-standalone-toolchain.sh --platform=android-14 --install-dir=/home/xxx/opt/ndk_standalone --ndk-dir=/home/xxx/repos/android-ndk-r9b`
    1. `cd ..`
1. `mkdir rust/build`
1. `cd rust/build`
1. 最新版の tag を確認
    - `git tag` 
1. 最新版をcheckout ( 執筆時: 1.11.0 )
    - `git checkout 1.11.0`
1. configure
    - Android NDK 対応: <br>`../configure --target=arm-linux-androideabi --android-cross-path=/home/xxx/opt/ndk_standalone`
    - Android NDK 不要: <br>`../configure`
1. `make`
1. `sudo make install`

### おまけ情報

- `time make -j9` を `i7-4770` で `22分`程度 掛かるみたい。
- `master` 最新は`make`に失敗するか、`make`できても`sudo make install`で失敗する可能性がある。
    - どうしても必要なコミットがタグ付けされた後に見つかる場合は、`git checkout -b my_custom_version_1`とかブランチを切って`git cherry-pick <commit>`で必要なコミットだけを回収するといいんじゃないかな。

### この文章と Rust のバージョン

- 執筆: Rust-0.11.0

# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Your Name <youremail@domain.com>
pkgname=punch-git
pkgver=$(git describe --tags)
pkgrel=1
epoch=
pkgdesc="file and folder management written in rust"
arch=(x86_64)
url="https://github.com/spicylemonade/punch.git"
license=('MIT')
groups=()
depends=(rustup)
makedepends=()
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("git+$url")
noextract=()
md5sums=('SKIP')
validpgpkeys=()

package() {
	cd "punch"
	. ./build.sh
}

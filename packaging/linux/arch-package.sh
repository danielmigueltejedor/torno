#!/usr/bin/env bash
# Repackages a Linux release tarball as an Arch Linux package (.pkg.tar.zst).
#
#   packaging/linux/arch-package.sh TARBALL ARCH [OUT_DIR]
#
# TARBALL is package.sh's torno-<version>-linux-<arch>.tar.gz (one top directory holding bin/ and
# share/). ARCH is x86_64 or aarch64. The package is written to OUT_DIR (default dist/release) as
# torno-<version>-linux-<arch>.pkg.tar.zst and is named after the tarball's .desktop file.
#
# makepkg runs as an unprivileged user in an archlinux:base-devel container. It only copies the
# prebuilt binaries, so both architectures are packaged on an x86_64 host.
set -euo pipefail

TARBALL="${1:?usage: arch-package.sh TARBALL ARCH [OUT_DIR]}"
ARCH="${2:?usage: arch-package.sh TARBALL ARCH [OUT_DIR]}"
OUT_DIR="${3:-dist/release}"
IMAGE="${ARCH_IMAGE:-archlinux:base-devel}"

case "$ARCH" in
  x86_64 | aarch64) ;;
  *) echo "error: unsupported arch $ARCH" >&2; exit 2 ;;
esac
[ -f "$TARBALL" ] || { echo "error: $TARBALL not found" >&2; exit 1; }

name="$(basename "$TARBALL" .tar.gz)"
version="${name#torno-}"
version="${version%-linux-"$ARCH"}"
if [ -z "$version" ] || [ "$version" = "$name" ]; then
  echo "error: $TARBALL is not named torno-<version>-linux-$ARCH.tar.gz" >&2
  exit 1
fi
# An Arch version can't hold a hyphen: 0.40.0-rc.1 becomes 0.40.0rc.1, which pacman sorts before
# 0.40.0. The file keeps the release's version.
pkgver="${version//-/}"

desktop="$(tar -tzf "$TARBALL" | sed -n 's|^[^/]*/share/applications/\([^/]*\)\.desktop$|\1|p' | head -n 1)"
[ -n "$desktop" ] || { echo "error: no .desktop file in $TARBALL" >&2; exit 1; }

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cp "$TARBALL" "$work/source.tar.gz"

cat >"$work/PKGBUILD" <<EOF
pkgname=$desktop
pkgver=$pkgver
pkgrel=1
pkgdesc='Parametric 3D CAD'
arch=('$ARCH')
url='https://github.com/getnodalia/torno'
license=('MIT OR Apache-2.0')
depends=('glibc' 'gcc-libs' 'libxkbcommon' 'libxkbcommon-x11' 'wayland' 'libx11' 'libxcb' 'libxcursor' 'libxi')
optdepends=('vulkan-icd-loader: Vulkan rendering'
            'mesa: Vulkan and OpenGL drivers'
            'xdg-desktop-portal: file dialogs')
provides=('torno')
conflicts=('torno')
options=('!strip' '!debug')
source=('source.tar.gz')
sha256sums=('SKIP')

package() {
  mkdir -p "\$pkgdir/usr"
  cp -R "\$srcdir/$name/." "\$pkgdir/usr/"
}
EOF

docker run --rm -v "$work:/build" -e CARCH="$ARCH" -e OWNER="$(id -u):$(id -g)" "$IMAGE" bash -euo pipefail -c '
  useradd -m builder
  chown -R builder /build
  # Hand the work directory back, or the caller can neither read the package nor delete it.
  trap "chown -R \"\$OWNER\" /build" EXIT
  # makepkg checks the package architecture against CARCH from its config.
  sed "s/^CARCH=.*/CARCH=\"$CARCH\"/" /etc/makepkg.conf >/tmp/makepkg.conf
  su builder -c "cd /build && PKGEXT=.pkg.tar.zst makepkg --config /tmp/makepkg.conf --nodeps --noconfirm"
'

pkg="$(find "$work" -maxdepth 1 -name "$desktop-$pkgver-1-$ARCH.pkg.tar.zst" | head -n 1)"
[ -n "$pkg" ] || { echo "error: makepkg wrote no package" >&2; ls -la "$work" >&2; exit 1; }
mkdir -p "$OUT_DIR"
cp "$pkg" "$OUT_DIR/torno-$version-linux-$ARCH.pkg.tar.zst"
echo "wrote $OUT_DIR/torno-$version-linux-$ARCH.pkg.tar.zst"

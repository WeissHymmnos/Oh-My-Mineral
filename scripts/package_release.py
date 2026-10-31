#!/usr/bin/env python3
"""
Build and package Mineral GUI + bundled Mineral Daemon into:
  1. dist/mineral-gui-0.1.0-1.x86_64.rpm
  2. dist/mineral-gui_0.1.0-1_amd64.deb
  3. dist/mineral-gui-0.1.0-linux-x86_64.tar.gz
"""

import hashlib
import os
import shutil
import stat
import struct
import subprocess
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
PKG_NAME = "mineral-gui"
PKG_VERSION = "0.1.0"
PKG_RELEASE = "1"

DESKTOP_ENTRY = """[Desktop Entry]
Type=Application
Name=Mineral Music Player
GenericName=Music Player
Comment=Material 3 Expressive Desktop Client & Daemon for Mineral
Exec=/usr/bin/mineral-gui-qt
Icon=mineral-gui
Terminal=false
Categories=AudioVideo;Audio;Player;Qt;
Keywords=Music;Player;Mineral;NetEase;Bilibili;Audio;
StartupWMClass=mineral-gui-qt
"""

APP_ICON_SVG = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="#EA957A"/>
      <stop offset="100%" stop-color="#E07A5F"/>
    </linearGradient>
  </defs>
  <rect width="128" height="128" rx="32" fill="url(#bg)"/>
  <circle cx="64" cy="64" r="38" fill="rgba(255,255,255,0.22)"/>
  <path d="M52 44 L86 64 L52 84 Z" fill="#FFFFFF"/>
</svg>
"""

SYSTEMD_SERVICE = """[Unit]
Description=Mineral Music Playback Daemon
After=network.target sound.target

[Service]
Type=simple
ExecStart=/usr/bin/mineral serve
Restart=on-failure
RestartSec=2

[Install]
WantedBy=default.target
"""


def find_mineral_daemon() -> Path:
    candidates = [
        Path.home() / ".cargo" / "bin" / "mineral",
        Path("/usr/bin/mineral"),
        Path("/usr/local/bin/mineral"),
    ]
    which_p = shutil.which("mineral")
    if which_p:
        candidates.insert(0, Path(which_p))
    for c in candidates:
        if c.is_file():
            return c
    raise FileNotFoundError("Could not find 'mineral' daemon binary")


def build_rpm_header_block(entries: list[tuple[int, int, int, bytes]], region_tag: int) -> bytes:
    """
    Build an RPM header section with an immutable region tag (62 or 63).
    entries: list of (tag, dtype, count, raw_bytes), sorted by tag ascending.
    RPM data types:
      3 = INT16 (2-byte aligned)
      4 = INT32 (4-byte aligned)
      6 = STRING (1-byte aligned, NUL-terminated)
      7 = BIN (1-byte aligned)
      8 = STRING_ARRAY (1-byte aligned, NUL-terminated strings)
    """
    entries = sorted(entries, key=lambda x: x[0])
    index_list = []
    store = bytearray()

    for tag, dtype, count, raw in entries:
        if dtype == 3:
            while len(store) % 2 != 0:
                store.append(0)
        elif dtype == 4:
            while len(store) % 4 != 0:
                store.append(0)
        offset = len(store)
        store.extend(raw)
        index_list.append((tag, dtype, offset, count))

    # Region tag (type 7 = BIN, count = 16 bytes) goes first in the index table,
    # and its 16-byte trailer sits at the very end of store.
    total_tags = len(index_list) + 1
    region_offset = len(store)
    neg_offset = -(total_tags * 16)
    trailer = struct.pack("!iiiI", region_tag, 7, neg_offset, 16)
    store.extend(trailer)

    idx_bytes = bytearray()
    idx_bytes.extend(struct.pack("!IIii", region_tag, 7, region_offset, 16))
    for tag, dtype, offset, count in index_list:
        idx_bytes.extend(struct.pack("!IIii", tag, dtype, offset, count))

    header_intro = struct.pack("!4sII", b"\x8e\xad\xe8\x01", 0, total_tags) + struct.pack("!I", len(store))
    return header_intro + bytes(idx_bytes) + bytes(store)


def pack_str(s: str) -> tuple[int, int, bytes]:
    return (6, 1, s.encode("utf-8") + b"\x00")


def pack_str_array(arr: list[str]) -> tuple[int, int, bytes]:
    return (8, len(arr), b"".join(s.encode("utf-8") + b"\x00" for s in arr))


def pack_int32(vals: list[int]) -> tuple[int, int, bytes]:
    return (4, len(vals), b"".join(struct.pack("!I", v & 0xFFFFFFFF) for v in vals))


def pack_int16(vals: list[int]) -> tuple[int, int, bytes]:
    return (3, len(vals), b"".join(struct.pack("!H", v & 0xFFFF) for v in vals))


def create_rpm(stage_dir: Path, out_rpm: Path, build_time: int) -> None:
    # Collect files & directories in sorted order
    rel_paths: list[Path] = []
    for root, dirs, files in os.walk(stage_dir):
        dirs.sort()
        files.sort()
        rpath = Path(root).relative_to(stage_dir)
        # Only include leaf/package-owned directories and files
        for f in files:
            rel_paths.append(rpath / f)
    rel_paths.sort(key=lambda p: "/" + p.as_posix())

    dir_map: dict[str, int] = {}
    dir_names: list[str] = []
    dir_indexes: list[int] = []
    base_names: list[str] = []
    file_sizes: list[int] = []
    file_modes: list[int] = []
    file_rdevs: list[int] = []
    file_mtimes: list[int] = []
    file_digests: list[str] = []
    file_linktos: list[str] = []
    file_flags: list[int] = []
    file_users: list[str] = []
    file_groups: list[str] = []
    file_devices: list[int] = []
    file_inodes: list[int] = []
    file_langs: list[str] = []

    total_size = 0
    for idx, rel in enumerate(rel_paths, start=1):
        full = stage_dir / rel
        st = full.lstat()
        parent_str = "/" + rel.parent.as_posix().lstrip(".")
        if not parent_str.endswith("/"):
            parent_str += "/"
        if parent_str not in dir_map:
            dir_map[parent_str] = len(dir_names)
            dir_names.append(parent_str)
        dir_indexes.append(dir_map[parent_str])
        base_names.append(rel.name)
        sz = st.st_size
        total_size += sz
        file_sizes.append(sz)
        file_modes.append(st.st_mode)
        file_rdevs.append(0)
        file_mtimes.append(build_time)
        digest = hashlib.sha256(full.read_bytes()).hexdigest()
        file_digests.append(digest)
        file_linktos.append("")
        is_doc = 16 if "/share/doc/" in ("/" + rel.as_posix()) else 0
        file_flags.append(is_doc)
        file_users.append("root")
        file_groups.append("root")
        file_devices.append(1)
        file_inodes.append(idx)
        file_langs.append("")

    # Build uncompressed cpio archive (newc format) with ./usr/... paths
    cpio_list = "\n".join("./" + p.as_posix() for p in rel_paths) + "\n"
    cpio_proc = subprocess.run(
        ["cpio", "-o", "-H", "newc", "--quiet"],
        input=cpio_list.encode("utf-8"),
        cwd=stage_dir,
        stdout=subprocess.PIPE,
        check=True,
    )
    raw_cpio = cpio_proc.stdout
    xz_proc = subprocess.run(
        ["xz", "-6", "-c"],
        input=raw_cpio,
        stdout=subprocess.PIPE,
        check=True,
    )
    xz_payload = xz_proc.stdout

    payload_sha256 = hashlib.sha256(xz_payload).hexdigest()
    uncompressed_sha256 = hashlib.sha256(raw_cpio).hexdigest()

    meta_entries: list[tuple[int, int, int, bytes]] = []

    def add_meta(tag: int, packed: tuple[int, int, bytes]) -> None:
        meta_entries.append((tag, packed[0], packed[1], packed[2]))

    add_meta(100, pack_str_array(["C"]))
    add_meta(1000, pack_str(PKG_NAME))
    add_meta(1001, pack_str(PKG_VERSION))
    add_meta(1002, pack_str(PKG_RELEASE))
    add_meta(1004, pack_str("Mineral Music Player GUI (Qt6/GTK4) and Bundled Playback Daemon"))
    add_meta(
        1005,
        pack_str(
            "Mineral desktop music player (Qt 6 & GTK 4 shells), protocol bridge (mineral-gui-bridge), "
            "and bundled Mineral playback daemon (mineral 0.5.10)."
        ),
    )
    add_meta(1006, pack_int32([build_time]))
    add_meta(1007, pack_str("localhost"))
    add_meta(1009, pack_int32([total_size]))
    add_meta(1014, pack_str("AGPL-3.0-only"))
    add_meta(1016, pack_str("Productivity/Multimedia/Sound/Players"))
    add_meta(1020, pack_str("https://github.com/mineral-music/mineral"))
    add_meta(1021, pack_str("linux"))
    add_meta(1022, pack_str("x86_64"))
    add_meta(1028, pack_int32(file_sizes))
    add_meta(1030, pack_int16(file_modes))
    add_meta(1033, pack_int16(file_rdevs))
    add_meta(1034, pack_int32(file_mtimes))
    add_meta(1035, pack_str_array(file_digests))
    add_meta(1036, pack_str_array(file_linktos))
    add_meta(1037, pack_int32(file_flags))
    add_meta(1039, pack_str_array(file_users))
    add_meta(1040, pack_str_array(file_groups))
    add_meta(1047, pack_str_array([PKG_NAME, "mineral", f"{PKG_NAME}(x86-64)"]))
    req_names = [
        "rpmlib(CompressedFileNames)",
        "rpmlib(FileDigests)",
        "rpmlib(PayloadFilesHavePrefix)",
        "rpmlib(PayloadIsXz)",
    ]
    req_vers = ["3.0.4-1", "4.6.0-1", "4.0-1", "5.2-1"]
    req_flags = [0x0100000A] * len(req_names)
    add_meta(1049, pack_str_array(req_names))
    add_meta(1050, pack_str_array(req_vers))
    add_meta(1051, pack_int32(req_flags))
    add_meta(1095, pack_int32(file_devices))
    add_meta(1096, pack_int32(file_inodes))
    add_meta(1097, pack_str_array(file_langs))
    add_meta(1112, pack_int32([8, 8, 8]))
    add_meta(1113, pack_str_array([f"{PKG_VERSION}-{PKG_RELEASE}", "0.5.10", f"{PKG_VERSION}-{PKG_RELEASE}"]))
    add_meta(1116, pack_int32(dir_indexes))
    add_meta(1117, pack_str_array(base_names))
    add_meta(1118, pack_str_array(dir_names))
    add_meta(1124, pack_str("cpio"))
    add_meta(1125, pack_str("xz"))
    add_meta(1126, pack_str("6"))
    add_meta(1132, pack_str("x86_64-suse-linux"))
    add_meta(5011, pack_int32([8]))
    add_meta(5092, pack_str_array([payload_sha256]))
    add_meta(5093, pack_int32([8]))
    add_meta(5097, pack_str_array([uncompressed_sha256]))

    meta_header = build_rpm_header_block(meta_entries, region_tag=63)

    # Signature header
    sha1_hdr = hashlib.sha1(meta_header).hexdigest()
    sha256_hdr = hashlib.sha256(meta_header).hexdigest()
    md5_all = hashlib.md5(meta_header + xz_payload).digest()
    sig_entries: list[tuple[int, int, int, bytes]] = [
        (269, *pack_str(sha1_hdr)),
        (273, *pack_str(sha256_hdr)),
        (1000, *pack_int32([len(meta_header) + len(xz_payload)])),
        (1004, 7, 16, md5_all),
        (1007, *pack_int32([len(raw_cpio)])),
    ]
    sig_header = bytearray(build_rpm_header_block(sig_entries, region_tag=62))
    while len(sig_header) % 8 != 0:
        sig_header.append(0)

    # 96-byte Lead
    pkg_full_name = f"{PKG_NAME}-{PKG_VERSION}-{PKG_RELEASE}.x86_64".encode("utf-8")[:65].ljust(66, b"\x00")
    lead = struct.pack(
        "!4sBBhh66shh16s",
        b"\xed\xab\xee\xdb",
        3,
        0,
        0,
        1,  # x86_64
        pkg_full_name,
        1,  # Linux
        5,  # RPMSIGTYPE_HEADERSIG
        b"\x00" * 16,
    )

    out_rpm.write_bytes(lead + bytes(sig_header) + meta_header + xz_payload)


def create_deb(stage_dir: Path, out_deb: Path, build_time: int) -> None:
    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        ctrl_dir = tmp_path / "control"
        ctrl_dir.mkdir()

        installed_kb = 0
        md5_lines = []
        for root, dirs, files in os.walk(stage_dir):
            dirs.sort()
            files.sort()
            for f in files:
                full = Path(root) / f
                rel = full.relative_to(stage_dir).as_posix()
                installed_kb += (full.stat().st_size + 1023) // 1024
                md5_hex = hashlib.md5(full.read_bytes()).hexdigest()
                md5_lines.append(f"{md5_hex}  {rel}")

        control_text = f"""Package: {PKG_NAME}
Version: {PKG_VERSION}-{PKG_RELEASE}
Section: sound
Priority: optional
Architecture: amd64
Provides: mineral
Installed-Size: {installed_kb}
Maintainer: Mineral Developers <dev@mineral.local>
Description: Mineral Music Player GUI (Qt6/GTK4) and Bundled Playback Daemon
 Material 3 Expressive desktop client (Qt 6 & GTK 4), protocol bridge
 (mineral-gui-bridge), and bundled Mineral playback daemon (mineral 0.5.10).
"""
        (ctrl_dir / "control").write_text(control_text, encoding="utf-8")
        (ctrl_dir / "md5sums").write_text("\n".join(md5_lines) + "\n", encoding="utf-8")

        control_tar = tmp_path / "control.tar.xz"
        data_tar = tmp_path / "data.tar.xz"
        deb_bin = tmp_path / "debian-binary"
        deb_bin.write_bytes(b"2.0\n")

        subprocess.run(
            ["tar", "--owner=0", "--group=0", "--numeric-owner", "-C", str(ctrl_dir), "-cJf", str(control_tar), "."],
            check=True,
        )
        subprocess.run(
            ["tar", "--owner=0", "--group=0", "--numeric-owner", "-C", str(stage_dir), "-cJf", str(data_tar), "."],
            check=True,
        )
        if out_deb.exists():
            out_deb.unlink()
        subprocess.run(
            ["ar", "rcsD", str(out_deb), str(deb_bin), str(control_tar), str(data_tar)],
            check=True,
        )


def main() -> None:
    env = os.environ.copy()
    cargo_bin = str(Path.home() / ".cargo" / "bin")
    env["PATH"] = f"{cargo_bin}:{env.get('PATH', '')}"

    print("==> Building workspace in --release mode...")
    subprocess.run(["cargo", "build", "--workspace", "--release"], cwd=ROOT, env=env, check=True)

    mineral_bin = find_mineral_daemon()
    print(f"==> Bundling mineral daemon from: {mineral_bin}")

    DIST.mkdir(parents=True, exist_ok=True)
    build_time = int(time.time())

    with tempfile.TemporaryDirectory() as tmp:
        stage = Path(tmp) / "rootfs"
        bin_dir = stage / "usr" / "bin"
        app_dir = stage / "usr" / "share" / "applications"
        icon_svg_dir = stage / "usr" / "share" / "icons" / "hicolor" / "scalable" / "apps"
        icon_512_dir = stage / "usr" / "share" / "icons" / "hicolor" / "512x512" / "apps"
        icon_256_dir = stage / "usr" / "share" / "icons" / "hicolor" / "256x256" / "apps"
        icon_128_dir = stage / "usr" / "share" / "icons" / "hicolor" / "128x128" / "apps"
        pixmap_dir = stage / "usr" / "share" / "pixmaps"
        doc_dir = stage / "usr" / "share" / "doc" / PKG_NAME
        systemd_dir = stage / "usr" / "lib" / "systemd" / "user"

        for d in (bin_dir, app_dir, icon_svg_dir, icon_512_dir, icon_256_dir, icon_128_dir, pixmap_dir, doc_dir, systemd_dir):
            d.mkdir(parents=True, exist_ok=True)

        bins_to_copy = [
            (mineral_bin, "mineral"),
            (ROOT / "target" / "release" / "mineral-gui-qt", "mineral-gui-qt"),
            (ROOT / "target" / "release" / "mineral-gui-gtk", "mineral-gui-gtk"),
            (ROOT / "target" / "release" / "mineral-gui-bridge", "mineral-gui-bridge"),
        ]
        for src, dst_name in bins_to_copy:
            dst = bin_dir / dst_name
            shutil.copy2(src, dst)
            subprocess.run(["strip", "--strip-unneeded", str(dst)], check=True)
            dst.chmod(0o755)

        (app_dir / "mineral-gui.desktop").write_text(DESKTOP_ENTRY, encoding="utf-8")
        (app_dir / "mineral-gui.desktop").chmod(0o644)

        assets_dir = ROOT / "assets"
        if (assets_dir / "logo.svg").is_file():
            shutil.copy2(assets_dir / "logo.svg", icon_svg_dir / "mineral-gui.svg")
        else:
            (icon_svg_dir / "mineral-gui.svg").write_text(APP_ICON_SVG, encoding="utf-8")
        (icon_svg_dir / "mineral-gui.svg").chmod(0o644)

        if (assets_dir / "logo.png").is_file():
            shutil.copy2(assets_dir / "logo.png", icon_512_dir / "mineral-gui.png")
            shutil.copy2(assets_dir / "logo.png", pixmap_dir / "mineral-gui.png")
            (icon_512_dir / "mineral-gui.png").chmod(0o644)
            (pixmap_dir / "mineral-gui.png").chmod(0o644)
        if (assets_dir / "logo_256.png").is_file():
            shutil.copy2(assets_dir / "logo_256.png", icon_256_dir / "mineral-gui.png")
            (icon_256_dir / "mineral-gui.png").chmod(0o644)
        if (assets_dir / "logo_128.png").is_file():
            shutil.copy2(assets_dir / "logo_128.png", icon_128_dir / "mineral-gui.png")
            (icon_128_dir / "mineral-gui.png").chmod(0o644)

        (systemd_dir / "mineral.service").write_text(SYSTEMD_SERVICE, encoding="utf-8")
        (systemd_dir / "mineral.service").chmod(0o644)
        shutil.copy2(ROOT / "README.md", doc_dir / "README.md")
        (doc_dir / "README.md").chmod(0o644)

        # Also update portable folder & tar.gz with bundled stripped `mineral` daemon & icon
        portable_dir = DIST / f"{PKG_NAME}-{PKG_VERSION}-linux-x86_64"
        if portable_dir.exists():
            shutil.rmtree(portable_dir)
        portable_dir.mkdir(parents=True)
        for bname in ("mineral", "mineral-gui-qt", "mineral-gui-gtk", "mineral-gui-bridge"):
            shutil.copy2(bin_dir / bname, portable_dir / bname)
        if (assets_dir / "logo.png").is_file():
            shutil.copy2(assets_dir / "logo.png", portable_dir / "mineral-gui.png")
        shutil.copy2(ROOT / "README.md", portable_dir / "README.md")

        tar_out = DIST / f"{PKG_NAME}-{PKG_VERSION}-linux-x86_64.tar.gz"
        subprocess.run(
            ["tar", "-C", str(DIST), "-czf", str(tar_out), portable_dir.name],
            check=True,
        )

        rpm_out = DIST / f"{PKG_NAME}-{PKG_VERSION}-{PKG_RELEASE}.x86_64.rpm"
        deb_out = DIST / f"{PKG_NAME}_{PKG_VERSION}-{PKG_RELEASE}_amd64.deb"

        print(f"==> Generating RPM package: {rpm_out}")
        create_rpm(stage, rpm_out, build_time)

        print(f"==> Generating DEB package: {deb_out}")
        create_deb(stage, deb_out, build_time)

    print("==> Done!")


if __name__ == "__main__":
    main()

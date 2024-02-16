(concatenate-manifests
 (list
  (packages->manifest
   (list `(,(@@ (binary packages rust) rust-nightly-bin) "rust-analyzer-preview")
	 `(,(@@ (binary packages rust) rust-nightly-bin) "rustfmt-preview")
	 `(,(@@ (binary packages rust) rust-nightly-bin) "cargo")
	 (@@ (binary packages rust) rust-nightly-bin)
	 ))
  (package->development-manifest
   (specification->package "emacs-next-pgtk"))
  (specifications->manifest
   (list "git"
	 "git:send-email"
	 "git-cal"
	 "gnupg"
	 "perl"
	 "cmake"
	 "python"

	 ;;x11 deps
	 "libxcursor" "libxrandr" "libxi" "xorg-server-xwayland" "xcb-util"

	 "libwebp"
	 "mesa"
	 "wayland"
	 "libxkbcommon"

	 "tree-sitter"

	 "emacs-rustic"
	 "emacs-realgud"
	 "gsettings-desktop-schemas"

	 "rust-cbindgen"
	 ;; for bindgen
	 "clang"
	 "gcc-toolchain"
	 ;; lsp
	 "bear"
	 "gdb"
	 "valgrind"
	 "strace"
	 "glibc:debug")
   )))

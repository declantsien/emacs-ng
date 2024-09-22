(use-modules (rustup build toolchain))

(concatenate-manifests
 (list
  (packages->manifest
   (list (rustup "nightly-2024-01-25" #:components (list 'rust-std 'rust-src 'rust-analyzer))))
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
	 "ninja"

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
	 ;; "libgccjit@11"
	 "gcc-toolchain"
	 ;; lsp
	 "bear"
	 "gdb"
	 "valgrind"
	 "strace"
	 "glibc:debug")
   )))

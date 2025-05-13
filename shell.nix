let 
	pkgs = import <nixpkgs> {};
	libPath =  with pkgs; pkgs.lib.makeLibraryPath [
		# wayland
		# libxkbcommon
		# vulkan-headers
		# vulkan-loader
		# libGL
	];
in
pkgs.mkShell {
	nativeBuildInputs = with pkgs; [
		cargo
		rustc
		rustfmt
		rust-analyzer
	];
	buildInputs = with pkgs; [
		# wayland
	];

	env = {
		RUST_BACKTRACE="1"; # full
		# LD_LIBRARY_PATH = libPath;
		RUSTFLAGS = "-C link-arg=-Wl,-rpath,${libPath}";
	};
}

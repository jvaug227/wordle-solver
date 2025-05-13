{
	pkgs,
	# makeWrapper,
	# patchelf

}:
# let
# 	dlopenLibraries = [
# 	];
# in
pkgs.rustPlatform.buildRustPackage {
	pname = "wordle-solver";
	version = "0.1.0";
	cargoLock.lockFile = ./Cargo.lock;
	src = pkgs.lib.cleanSource ./.;

	nativeBuildInputs = [
		# patchelf
		# makeWrapper
	];

	# buildInputs = [
	# ] ++ dlopenLibraries;

	# patchelf --shrink-rpath in phaseFixup just removes the paths as for some reason these are not declared as necessary.
	# RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath dlopenLibraries}";

	# Patch using patchelf after Fixup to circumvent the rpaths being stripped
	# postFixup = ''
	# 	patchelf $out/bin/NAME --add-rpath ${pkgs.lib.makeLibraryPath dlopenLibraries}
	# '';

	# Also a solid solution to fixing the dlopens
	# postInstall = ''
	# 	wrapProgram $out/bin/sbi \
	# 	--prefix LD_LIBRARY_PATH : ${ pkgs.lib.makeLibraryPath [ libxkbcommon wayland vulkan-loader libGL ] }
	# '';
}

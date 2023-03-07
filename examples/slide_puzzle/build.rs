// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

fn main() {
    let config = slint_build::CompilerConfiguration::new();
    #[cfg(feature = "mcu-board-support")]
    let config = config.embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("slide_puzzle.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}

use indicatif::ProgressStyle;

pub fn get_bar_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template(
            "{prefix:>12.cyan.bold} [{bar:40.green/cyan}] {bytes}/{total_bytes} {msg:.green.bold}",
        )
        .progress_chars("=> ")
}

pub fn get_spinner_style() -> ProgressStyle {
    ProgressStyle::default_spinner()
        .tick_strings(&[
            "====                                    ",
            "  ====                                  ",
            "    ====                                ",
            "      ====                              ",
            "        ====                            ",
            "          ====                          ",
            "            ====                        ",
            "              ====                      ",
            "                ====                    ",
            "                  ====                  ",
            "                    ====                ",
            "                      ====              ",
            "                        ====            ",
            "                          ====          ",
            "                            ====        ",
            "                              ====      ",
            "                                ====    ",
            "                                  ====  ",
            "                                    ====",
            "========================================",
        ])
        .template("{prefix:>12.cyan.bold} [{spinner:40.green}] {elapsed_precise} {msg:.green.bold}")
}

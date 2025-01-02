use colored::Colorize;

pub fn print_splash() {
    println!(
        r#"{}{}{}"#,"
                                            ...
                                            ....
                                           .......
                                         ...............
                                ......................
                        ...........................
               .........................    ......
       ....................                   ....
..............                                 ..
"
        .bright_green()
        .bold(),
        r#"
      ######   ######   ########   #######  ########
     ##       ##    ##  #  ##  #  ##           ##   
     ##       ##    ##  #  ##  #  ########     ##   
     ##       ##    ##  #  ##  #  ##           ##   
      ######   ######   #  ##  #   #######     ##   

    A transparent, extensible, and declarative package manager
            for your favorite Operating Systems.
                      Server Version "#
            .bright_white()
            .bold(),
        env!("CARGO_PKG_VERSION").green().italic().bold(),
    )
}
